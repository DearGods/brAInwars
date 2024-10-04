use crate::configuration::settings::Settings;
use crate::envars::app_env_var::AppEnvVar;
use crate::envars::env_var;
use crate::envars::get_env_var_or_panic::get_env_var_or_panic;
use crate::middlewares::authentication::{authentication_layer, Backend};
use crate::{routes, websockets};
use axum::routing::{get, post};
use axum::{Extension, Router};
use axum_login::login_required;
#[cfg(not(feature = "my-test"))]
use solana_client::nonblocking::rpc_client::RpcClient;
#[cfg(feature = "my-test")]
use solana_program_test::BanksClient;
use solana_sdk::signature::Keypair;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
#[cfg(feature = "my-test")]
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

pub struct Application {
    app: Router,
    listener: TcpListener,
}

// Our shared state
#[cfg(not(feature = "my-test"))]
pub struct AppState {
    // Channel used to send messages to all connected clients.
    // https://github.com/tokio-rs/axum/blob/main/examples/chat/src/main.rs
    pub game_tx: broadcast::Sender<String>,
    pub chat_tx: broadcast::Sender<String>,
    pub client: Arc<RpcClient>,
    pub keypair: Arc<Keypair>,
    pub pool: PgPool,
}

#[cfg(feature = "my-test")]
pub struct AppState {
    // Channel used to send messages to all connected clients.
    // https://github.com/tokio-rs/axum/blob/main/examples/chat/src/main.rs
    pub game_tx: broadcast::Sender<String>,
    pub chat_tx: broadcast::Sender<String>,
    pub client: Arc<Mutex<BanksClient>>,
    pub keypair: Arc<Keypair>,
    pub pool: PgPool,
}

#[derive(Clone)]
pub struct ApplicationBaseUrl(pub String);

impl ApplicationBaseUrl {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Application {
    pub async fn build(settings: Settings, app_state: Arc<AppState>, db_pool: PgPool) -> Self {
        let auth_layer = authentication_layer(&db_pool).await;

        let auth_router = Router::new()
            .route("/auth/protected", get(routes::auth::protected::handler))
            .route("/users/get", post(routes::users::get::handler))
            .route(
                "/users/update_name",
                post(routes::users::update_name::handler),
            )
            .route("/auth/logout", post(routes::auth::logout::handler))
            .route("/games/join_game", post(routes::games::join_game::handler))
            .route(
                "/games/leave_game",
                post(routes::games::leave_game::handler),
            )
            .route("/games/get", get(routes::games::get::handler));

        let un_auth_router = Router::new()
            .route("/games/unauth_get", get(routes::games::unauth_get::handler))
            .route("/auth/login", post(routes::auth::login::handler))
            .route("/health_check", get(routes::health_check::get::handler))
            .route("/games/index", get(routes::games::index::handler))
            .route("/users/create", post(routes::users::create::handler));

        let auth_websockets = Router::new().route(
            "/auth_chat_ws_notifier",
            get(websockets::auth_chat_ws_notifier::websocket_handler),
        );

        let un_auth_websockets = Router::new()
            .route(
                "/game_ws_notifier",
                get(websockets::game_ws_notifier::websocket_handler),
            )
            .route(
                "/unauth_chat_ws_notifier",
                get(websockets::unauth_chat_ws_notifier::websocket_handler),
            );

        let app_env = get_env_var_or_panic(AppEnvVar::AppEnvironment);
        let app_env = <env_var::EnvVar as AsRef<String>>::as_ref(&app_env);
        let cors = match app_env.as_str() {
            "local" => CorsLayer::very_permissive(),
            _ => CorsLayer::permissive(),
        };

        let application_base_url = ApplicationBaseUrl(settings.application.base_url.clone());
        let app = Router::new()
            .nest("/api/", auth_router)
            .nest("/api/", auth_websockets)
            .route_layer(login_required!(Backend, login_url = "/api/auth/login"))
            .nest("/api/", un_auth_router)
            .nest("/api/", un_auth_websockets)
            .nest_service(
                "/",
                ServeDir::new("./dist").fallback(ServeFile::new("./dist/index.html")),
            )
            .layer(auth_layer)
            // .layer(session_layer)
            // .layer(axum::middleware::from_fn(cors_headers))
            .layer(Extension(application_base_url))
            .layer(Extension(db_pool.clone()))
            .layer(cors)
            .with_state(app_state.clone());

        let listener = TcpListener::bind(settings.application.address())
            .await
            .unwrap();
        tracing::info!("Listening on {}", listener.local_addr().unwrap());
        Application { app, listener }
    }

    pub async fn run(self) -> std::io::Result<()> {
        axum::serve(self.listener, self.app).await
    }

    pub fn address(&self) -> String {
        format!("{}", self.listener.local_addr().unwrap())
    }

    pub fn port(&self) -> u16 {
        self.listener.local_addr().unwrap().port()
    }
}
