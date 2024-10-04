use crate::database::nonce::delete_nonce_by_date::delete_nonce_by_date;
use crate::workers::game_worker::ExecutionOutcome;
use chrono::Duration;
use sqlx::PgPool;

#[tracing::instrument(name = "delete_nonces", level = "trace", skip(pool), err)]
pub async fn delete_nonces(pool: &PgPool) -> Result<ExecutionOutcome, anyhow::Error> {
    let yesterday = chrono::Utc::now() - Duration::days(3);
    let mut transaction = pool.begin().await?;
    delete_nonce_by_date(&mut transaction, yesterday)
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
    transaction.commit().await?;
    Ok(ExecutionOutcome::TaskCompleted)
}
