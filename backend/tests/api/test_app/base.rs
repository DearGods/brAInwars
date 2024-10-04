use crate::test_app::helpers::{
    brain_wars_program_test, configure_database, move_clock, set_test_config_values,
};
use backend_lib::blockchain::solana::helpers::get_keypair;
use backend_lib::configuration::get_configuration::get_configuration;
use backend_lib::configuration::settings::Settings;
use backend_lib::domain::config::Config;
use backend_lib::envars::load_dotenv::load_dotenv;
use backend_lib::startup::application::{AppState, Application};
use backend_lib::telemetry::subscriber::{get_subscriber, init_subscriber};
use backend_lib::workers::game_worker::worker_loop;
use once_cell::sync::Lazy;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Keypair;
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    if env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("brain-wars_test", "debug", std::io::stdout, true);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber("brain-wars_test", "debug", std::io::sink, true);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
    pub api_client: reqwest::Client,
    pub configuration: Settings,
    pub keypair: Arc<Keypair>,
    pub configs: HashMap<String, serde_json::Value>,
    pub context: Arc<Mutex<ProgramTestContext>>,
    pub app_state: Arc<AppState>,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    load_dotenv();

    let configuration = {
        let mut configuration = get_configuration().expect("Failed to read configuration");
        configuration.application.port = 0;
        configuration.database.name = Uuid::new_v4().to_string();
        configuration
    };
    tracing::info!("Starting with configuration {:#?}", configuration);

    let db_pool = configure_database(&configuration.database).await;
    set_test_config_values(&db_pool).await;

    let configs = Config::get_configs_hashmap(&db_pool).await.unwrap();
    let keypair = Arc::new(get_keypair(None).unwrap());
    let (game_tx, _rx) = broadcast::channel(100);
    let (chat_tx, _rx) = broadcast::channel(100);
    let api_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    let program = brain_wars_program_test();
    let context = Arc::new(Mutex::new(program.start_with_context().await));

    let app_state = Arc::new(AppState {
        pool: db_pool.clone(),
        game_tx,
        chat_tx,
        client: Arc::new(Mutex::new(context.lock().await.banks_client.clone())),
        keypair: keypair.clone(),
    });

    let application =
        Application::build(configuration.clone(), app_state.clone(), db_pool.clone()).await;
    let address = format!("http://{}", application.address());
    let port = application.port();
    let _worker_task = tokio::spawn(worker_loop(db_pool.clone(), app_state.clone()));
    let _application_task = tokio::spawn(application.run());
    let _clock = tokio::spawn(move_clock(context.clone()));

    TestApp {
        address,
        port,
        db_pool,
        api_client,
        configuration,
        keypair,
        configs,
        context,
        app_state: app_state.clone(),
    }
}
