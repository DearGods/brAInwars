use crate::domain::job::job_obj::Job;
use crate::domain::job::job_status::JobStatus;
use crate::domain::job::job_type::JobType;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Get Job By JobType and Status", skip(transaction), ret, err)]
pub async fn get_job_by_id(
    transaction: &mut Transaction<'_, Postgres>,
    id: Uuid,
) -> anyhow::Result<Option<Job>> {
    let job = sqlx::query_as!(
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
        WHERE id = $1
        LIMIT 1
        "#,
        id
    )
    .fetch_optional(&mut **transaction)
    .await?;
    Ok(job)
}
