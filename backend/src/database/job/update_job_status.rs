use crate::domain::job::job_status::JobStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Update Job Status", skip(transaction), ret, err)]
pub async fn update_job_status(
    transaction: &mut Transaction<'_, Postgres>,
    status: JobStatus,
    id: Uuid,
    error: Option<String>,
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"UPDATE jobs SET status = $1 , error = $2 WHERE id = $3"#,
        status.to_string(),
        error,
        id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
