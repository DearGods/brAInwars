use crate::database::job::get_jobs_by_status::get_jobs_by_status;
use crate::database::job::update_job_status::update_job_status;
use crate::domain::config::Config;
use crate::domain::job::job_status::JobStatus;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::workers::game_worker::ExecutionOutcome;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;

#[tracing::instrument(name = "process_jobs", level = "trace", skip(pool, app_state), err)]
pub async fn process_jobs(
    pool: &PgPool,
    app_state: Arc<AppState>,
) -> Result<ExecutionOutcome, anyhow::Error> {
    let mut transaction = pool.begin().await?;
    let jobs = get_jobs_by_status(&mut transaction, JobStatus::Pending).await?;
    transaction.commit().await?;
    for mut job in jobs {
        let pool = pool.clone();
        let app_state = app_state.clone();
        let mut transaction = pool.begin().await?;
        update_job_status(&mut transaction, JobStatus::Running, job.id, None).await?;
        transaction.commit().await?;
        tokio::spawn(async move {
            let _ = job.run_job(&pool, app_state.clone()).await;
        });
    }
    Ok(ExecutionOutcome::TaskCompleted)
}

pub async fn job_loop(pool: PgPool, app_state: Arc<AppState>) -> Result<(), anyhow::Error> {
    let configs = Config::get_configs_hashmap(&pool)
        .await
        .map_err(Error::from)?;
    let worker_interval = Config::get_config_value(&configs, "worker_interval")?
        .as_u64()
        .ok_or(Error::FailedToCastConfigToKeyValue(
            "worker_interval".to_string(),
        ))?;
    loop {
        match process_jobs(&pool, app_state.clone()).await {
            Ok(ExecutionOutcome::EmptyQueue) => {}
            Ok(ExecutionOutcome::TaskCompleted) => {}
            Err(e) => {
                tracing::error!("worker_loop: process_jobs: error: {}", e);
                tokio::time::sleep(Duration::from_millis(worker_interval)).await;
            }
        }
        tokio::time::sleep(Duration::from_millis(worker_interval)).await;
    }
}
