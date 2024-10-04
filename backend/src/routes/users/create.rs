use crate::database::nonce::create_nonce::create_nonce;
use crate::database::user::create_user::create_user;
use crate::database::user::get_user_by_wallet::get_user_by_wallet;
use crate::database::user::user_exists::user_exists;
use crate::domain::id::Id;
use crate::domain::nonce::Nonce;
use crate::domain::secret::Secret;
use crate::domain::user::User;
use crate::errors::error::Error;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::database::nonce::get_or_create_nonce::get_or_create_nonce;
use crate::routes::basic_response::ResponseStatus;
use std::fmt::Display;
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub wallet_address: String,
}

impl Display for CreateUserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub id: Option<Id>,
    pub wallet_address: Option<String>,
    pub nonce: Option<String>,
    pub status: ResponseStatus,
    pub message: Option<String>,
}

#[tracing::instrument(name = "Create new user", skip(pool, body), err, ret)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, StatusCode> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let nonce = Secret::from(Nonce::generate_nonce());
    let user = User::new(&body.wallet_address).await?;
    if user_exists(&mut transaction, &body.wallet_address)
        .await
        .map_err(Error::from)?
    {
        let db_nonce = get_or_create_nonce(&mut transaction, &body.wallet_address, &nonce)
            .await
            .map_err(Error::from)?;
        let db_user = get_user_by_wallet(&mut transaction, &body.wallet_address)
            .await
            .map_err(Error::from)?;
        if body.wallet_address != user.wallet_address
            || db_user.wallet_address != body.wallet_address
        {
            return Err(StatusCode::from(Error::UserMismatch));
        }
        transaction.commit().await.map_err(Error::from)?;
        Ok(Json(CreateUserResponse {
            id: Option::from(Id::from(db_user.id)),
            nonce: Option::from(db_nonce.nonce.as_ref().to_string()),
            wallet_address: Option::from(user.wallet_address),
            status: ResponseStatus::Success,
            message: None,
        }))
    } else {
        create_user(&mut transaction, &user.wallet_address, &user.name).await?;
        create_nonce(&mut transaction, &user.wallet_address, &nonce).await?;
        transaction.commit().await.map_err(Error::from)?;
        Ok(Json(CreateUserResponse {
            id: Option::from(Id::from(user.id)),
            nonce: Option::from(nonce.as_ref().to_string()),
            wallet_address: Option::from(user.wallet_address),
            status: ResponseStatus::Success,
            message: None,
        }))
    }
}
