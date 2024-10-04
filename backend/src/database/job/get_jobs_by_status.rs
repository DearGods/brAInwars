use crate::domain::job::job_obj::Job;
use crate::domain::job::job_status::JobStatus;
use crate::domain::job::job_type::JobType;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(
    name = "Get Jobs By Status",
    level = "trace",
    skip(transaction),
    ret,
    err
)]
pub async fn get_jobs_by_status(
    transaction: &mut Transaction<'_, Postgres>,
    status: JobStatus,
) -> anyhow::Result<Vec<Job>> {
    let jobs = sqlx::query_as!(
        Job,
        r#"
        SELECT
        id,
        created_at,
        game_uuid,
        wallet_address,
        job_type as "job_type: JobType",
        status as "status: JobStatus",
        error,
        txn
        FROM jobs
        WHERE status = $1
        ORDER BY created_at ASC
        "#,
        status.to_string()
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(jobs)
}
