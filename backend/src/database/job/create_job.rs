use crate::domain::job::job_obj::Job;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "Create Job", skip(transaction), ret, err)]
pub async fn create_job(
    transaction: &mut Transaction<'_, Postgres>,
    job: Job,
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO jobs
        (
        id,
        created_at,
        game_uuid,
        wallet_address,
        job_type,
        status,
        error,
        txn
       )
       VALUES ( $1, $2 , $3 , $4 , $5 , $6 , $7 , $8)
       "#,
        job.id,
        job.created_at,
        job.game_uuid,
        job.wallet_address,
        job.job_type.to_string(),
        job.status.to_string(),
        job.error,
        job.txn
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
