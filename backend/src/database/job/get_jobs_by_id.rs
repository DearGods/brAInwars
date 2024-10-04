use crate::domain::job::job_obj::Job;
use crate::domain::job::job_status::JobStatus;
use crate::domain::job::job_type::JobType;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Get Jobs By Id", level = "trace", skip(transaction), ret, err)]
pub async fn get_jobs_by_game_uuid(
    transaction: &mut Transaction<'_, Postgres>,
    game_uuid: Uuid,
) -> anyhow::Result<Vec<Job>> {
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
        WHERE game_uuid = $1
        ORDER BY created_at ASC
        "#,
        game_uuid
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(job)
}
