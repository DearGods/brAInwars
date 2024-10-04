use crate::database::user::update_user_name::update_user_name;
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
pub struct UpdateUserNameRequest {
    pub wallet_address: String,
    pub name: String,
}

impl Display for UpdateUserNameRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserNameResponse {
    pub id: Option<Id>,
    pub wallet_address: String,
    pub name: String,
    pub status: ResponseStatus,
    pub message: Option<String>,
}

#[tracing::instrument(name = "Update user name", skip(pool, body), err, ret)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Json(body): Json<UpdateUserNameRequest>,
) -> Result<Json<UpdateUserNameResponse>, StatusCode> {
    let user = match auth.user {
        Some(user) => user,
        None => {
            return Ok(Json(UpdateUserNameResponse {
                id: None,
                wallet_address: body.wallet_address,
                name: "".to_string(),
                status: ResponseStatus::Failure,
                message: Some("User not found".to_string()),
            }));
        }
    };
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    update_user_name(&mut transaction, user.id, &body.name)
        .await
        .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(UpdateUserNameResponse {
        id: Option::from(Id::from(user.id)),
        wallet_address: user.wallet_address,
        name: body.name,
        status: ResponseStatus::Success,
        message: None,
    }))
}
