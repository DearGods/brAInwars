use crate::domain::secret::Secret;
use crate::errors::error::Error;
use anyhow::anyhow;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Create Nonce", skip(transaction), ret, err)]
pub async fn create_nonce(
    transaction: &mut Transaction<'_, Postgres>,
    wallet_address: &str,
    nonce: &Secret<String>,
) -> Result<(), Error> {
    let now = Utc::now();
    let uuid = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO nonces
        (id, created_at, wallet_address, nonce)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid,
        now,
        wallet_address,
        nonce.as_ref(),
    )
    .execute(&mut **transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create nonce: {:?}", e);
        Error::UnexpectedError(anyhow!("Failed to create nonce"))
    })?;
    Ok(())
}
