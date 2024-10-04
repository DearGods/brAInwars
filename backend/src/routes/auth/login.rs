use crate::blockchain::solana::helpers::validate_signature;
use crate::database::nonce::get_nonce::get_nonce;
use crate::database::user::get_user_by_wallet::get_user_by_wallet;
use crate::domain::id::Id;
use crate::errors::error::Error;
use crate::middlewares::authentication::{Backend, Credentials};
use crate::routes::basic_response::ResponseStatus;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_login::AuthSession;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use sqlx::PgPool;
use std::fmt::Display;
use std::str::FromStr;
use tracing;
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub wallet_address: String,
    pub signed_message: String,
    pub nonce: String,
}

impl Display for LoginRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub uuid: Option<Id>,
    pub wallet_address: Option<String>,
    pub name: Option<String>,
    pub status: ResponseStatus,
    pub message: Option<String>,
}

#[tracing::instrument(name = "Login", skip(pool, auth), ret, err)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(mut auth): Extension<AuthSession<Backend>>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let db_user = get_user_by_wallet(&mut transaction, &body.wallet_address)
        .await
        .map_err(Error::from)?;
    let db_nonce = get_nonce(&mut transaction, &body.wallet_address)
        .await
        .map_err(Error::from)?;
    if db_user.wallet_address != body.wallet_address || *db_nonce.nonce.as_ref() != body.nonce {
        return Ok(Json(LoginResponse {
            uuid: Option::from(Id::from(db_user.id)),
            wallet_address: Option::from(body.wallet_address),
            name: None,
            status: ResponseStatus::Failure,
            message: Option::from("User mismatch".to_string()),
        }));
    }
    let public_key = Pubkey::from_str(&body.wallet_address).map_err(Error::from)?;
    validate_signature(&body.nonce, &body.signed_message, &public_key).map_err(Error::from)?;

    let creds: Credentials = Credentials {
        wallet_address: body.wallet_address,
        signed_message: body.signed_message,
        nonce: body.nonce,
    };

    let user = match auth.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Ok(Json(LoginResponse {
                uuid: Option::from(Id::from(db_user.id)),
                wallet_address: Option::from(public_key.to_string()),
                name: None,
                status: ResponseStatus::Failure,
                message: Option::from("User mismatch".to_string()),
            }));
        }
        Err(_) => {
            return Ok(Json(LoginResponse {
                uuid: Option::from(Id::from(db_user.id)),
                wallet_address: Option::from(public_key.to_string()),
                name: None,
                status: ResponseStatus::Failure,
                message: Option::from("User mismatch".to_string()),
            }));
        }
    };

    if auth.login(&user).await.is_err() {
        return Ok(Json(LoginResponse {
            uuid: Option::from(Id::from(db_user.id)),
            wallet_address: Option::from(public_key.to_string()),
            name: None,
            status: ResponseStatus::Failure,
            message: Option::from("User mismatch".to_string()),
        }));
    }

    Ok(Json(LoginResponse {
        uuid: Option::from(Id::from(db_user.id)),
        wallet_address: Option::from(public_key.to_string()),
        name: Option::from(db_user.name),
        status: ResponseStatus::Success,
        message: None,
    }))
}
