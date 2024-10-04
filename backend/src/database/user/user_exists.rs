use crate::errors::error::Error;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "Check if user exists", skip(transaction), ret, err)]
pub async fn user_exists(
    transaction: &mut Transaction<'_, Postgres>,
    wallet_address: &str,
) -> Result<bool, Error> {
    match sqlx::query!(
        "SELECT id FROM users WHERE wallet_address = $1 LIMIT 1",
        wallet_address
    )
    .fetch_optional(&mut **transaction)
    .await?
    {
        None => Ok(false),
        Some(_) => Ok(true),
    }
}
