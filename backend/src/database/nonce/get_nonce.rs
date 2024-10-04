use crate::domain::nonce::Nonce;
use crate::domain::secret::Secret;
use crate::errors::error::Error;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "Get nonce", skip(transaction), level = "trace", ret, err)]
pub async fn get_nonce(
    transaction: &mut Transaction<'_, Postgres>,
    wallet_address: &str,
) -> Result<Nonce, Error> {
    match sqlx::query_as!(
        Nonce,
        r#"
        SELECT
        id,
        created_at,
        wallet_address,
        nonce as "nonce: Secret<String>"
        FROM nonces
        WHERE wallet_address = $1
        ORDER BY created_at DESC
        LIMIT 1"#,
        wallet_address
    )
    .fetch_optional(&mut **transaction)
    .await?
    {
        None => Err(Error::NonceNotFound),
        Some(i) => Ok(i),
    }
}
