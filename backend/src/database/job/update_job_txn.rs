use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Update Job Txn", skip(transaction), ret, err)]
pub async fn update_job_txn(
    transaction: &mut Transaction<'_, Postgres>,
    id: Uuid,
    txn: Option<String>,
) -> anyhow::Result<()> {
    sqlx::query!(r#"UPDATE jobs SET txn = $1 WHERE id = $2"#, txn, id)
        .execute(&mut **transaction)
        .await?;
    Ok(())
}
