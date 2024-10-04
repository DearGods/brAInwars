use crate::errors::error::Error;
use anyhow::anyhow;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Count Participants", skip(transaction), ret, err)]
pub(crate) async fn count_participants(
    transaction: &mut Transaction<'_, Postgres>,
    game_id: Uuid,
) -> Result<i64, Error> {
    let count = sqlx::query_scalar!(
        r#"SELECT COUNT(*) as count FROM participants WHERE game_id = $1"#,
        game_id,
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to count participants: {:?}", e);
        Error::UnexpectedError(anyhow!("Failed to count participants"))
    })?;
    Ok(count.unwrap_or(0))
}
