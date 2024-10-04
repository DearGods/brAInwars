use crate::errors::error::Error;
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "Get nonce", level = "trace", skip(transaction), ret, err)]
pub async fn delete_nonce_by_date(
    transaction: &mut Transaction<'_, Postgres>,
    date: DateTime<Utc>,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        DELETE
        FROM nonces
        WHERE created_at < $1
        "#,
        date
    )
    .execute(&mut **transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete_nonce_by_date: {:?}", e);
        Error::UnexpectedError(anyhow!("Failed to delete_nonce_by_date"))
    })?;
    Ok(())
}
