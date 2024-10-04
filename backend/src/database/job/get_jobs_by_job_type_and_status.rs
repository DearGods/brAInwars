use crate::domain::job::job_obj::Job;
use crate::domain::job::job_status::JobStatus;
use crate::domain::job::job_type::JobType;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "Get Jobs By JobType and Status", skip(transaction), ret, err)]
pub async fn get_jobs_by_job_type_and_status(
    transaction: &mut Transaction<'_, Postgres>,
    job_type: JobType,
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
        WHERE job_type = $1 AND status = $2
        ORDER BY created_at ASC
        "#,
        job_type.to_string(),
        status.to_string()
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(jobs)
}
