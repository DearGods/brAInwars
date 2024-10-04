use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::routes::basic_response::ResponseStatus;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_login::AuthSession;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub wallet_address: String,
}

impl Display for LogoutRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutResponse {
    pub status: ResponseStatus,
    pub message: Option<String>,
}

#[tracing::instrument(name = "Logout", skip(auth), ret, err)]
pub async fn handler(
    Extension(mut auth): Extension<AuthSession<Backend>>,
    Json(_): Json<LogoutRequest>,
) -> Result<Json<LogoutResponse>, StatusCode> {
    auth.logout()
        .await
        .map_err(|e| Error::AuthError(e.to_string()))?;
    Ok(Json(LogoutResponse {
        status: ResponseStatus::Success,
        message: None,
    }))
}
