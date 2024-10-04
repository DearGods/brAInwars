use crate::errors::error::Error;
use anyhow::anyhow;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Create User", skip(transaction), ret, err)]
pub async fn create_user(
    transaction: &mut Transaction<'_, Postgres>,
    wallet_address: &str,
    name: &str,
) -> Result<Uuid, Error> {
    let now = Utc::now();
    let uuid = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO users (id, created_at, wallet_address, name) VALUES ($1, $2, $3, $4)"#,
        uuid,
        now,
        wallet_address,
        name
    )
    .execute(&mut **transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create user: {:?}", e);
        Error::UnexpectedError(anyhow!("Failed to create user"))
    })?;
    Ok(uuid)
}
