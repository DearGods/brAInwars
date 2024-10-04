use crate::domain::nonce::Nonce;
use crate::domain::secret::Secret;
use crate::errors::error::Error;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "Get or create nonce",
    skip(transaction),
    level = "trace",
    ret,
    err
)]
pub async fn get_or_create_nonce(
    transaction: &mut Transaction<'_, Postgres>,
    wallet_address: &str,
    nonce: &Secret<String>,
) -> Result<Nonce, Error> {
    let now = Utc::now();
    let uuid = Uuid::new_v4();

    match sqlx::query_as!(
        Nonce,
        r#"
        INSERT INTO nonces
        (id, created_at, wallet_address, nonce)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (wallet_address)
        DO UPDATE set nonce = $4
        RETURNING
        id,
        created_at,
        wallet_address,
        nonce as "nonce: Secret<String>"
        "#,
        uuid,
        now,
        wallet_address,
        nonce.as_ref(),
    )
    .fetch_optional(&mut **transaction)
    .await?
    {
        None => Err(Error::NonceNotFound),
        Some(i) => Ok(i),
    }
}
