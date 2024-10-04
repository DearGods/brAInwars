use crate::database::user::get_user_by_wallet::get_user_by_wallet;
use crate::domain::id::Id;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::routes::basic_response::ResponseStatus;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_login::AuthSession;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::fmt::Display;
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserRequest {
    pub wallet_address: String,
}

impl Display for GetUserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub id: Option<Id>,
    pub wallet_address: String,
    pub name: String,
    pub status: ResponseStatus,
    pub message: Option<String>,
}

#[tracing::instrument(name = "Get user", skip(pool), ret, err)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Json(body): Json<GetUserRequest>,
) -> Result<Json<GetUserResponse>, StatusCode> {
    let user = match auth.user {
        Some(user) => user,
        None => {
            return Ok(Json(GetUserResponse {
                id: None,
                wallet_address: body.wallet_address,
                name: "".to_string(),
                status: ResponseStatus::Failure,
                message: Option::from("User not found".to_string()),
            }));
        }
    };
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let result = get_user_by_wallet(&mut transaction, &body.wallet_address)
        .await
        .map_err(Error::from)?;
    if result.id != user.id
        || result.wallet_address != user.wallet_address
        || body.wallet_address != user.wallet_address
    {
        return Ok(Json(GetUserResponse {
            id: None,
            wallet_address: body.wallet_address,
            name: "".to_string(),
            status: ResponseStatus::Failure,
            message: Option::from("User mismatch".to_string()),
        }));
    }
    Ok(Json(GetUserResponse {
        id: Option::from(Id::from(result.id)),
        wallet_address: result.wallet_address.to_string(),
        name: result.name,
        status: ResponseStatus::Success,
        message: None,
    }))
}
