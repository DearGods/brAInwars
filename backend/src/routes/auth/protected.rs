use crate::middlewares::authentication::Backend;
use axum::response::IntoResponse;
use axum::Extension;
use axum_login::AuthSession;
use http::StatusCode;

#[tracing::instrument(name = "Protected", skip(auth))]
pub async fn handler(Extension(auth): Extension<AuthSession<Backend>>) -> impl IntoResponse {
    match auth.user {
        Some(user) => { format!("Logged in as: {}", user.wallet_address) }.into_response(),
        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
