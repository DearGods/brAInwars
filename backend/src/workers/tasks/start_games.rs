use crate::database::game::get_games_by_status::get_games_by_status;
use crate::database::job::get_jobs_by_id::get_jobs_by_game_uuid;
use crate::domain::game::game_status::GameStatus;
use crate::domain::job::job_obj::Job;
use crate::domain::job::job_type::JobType;
use crate::startup::application::AppState;
use crate::workers::game_worker::ExecutionOutcome;
use sqlx::PgPool;
use std::sync::Arc;

#[tracing::instrument(name = "sync_games", level = "trace", skip(pool, _app_state), err)]
pub async fn start_games(
    pool: &PgPool,
    _app_state: Arc<AppState>,
) -> Result<ExecutionOutcome, anyhow::Error> {
    let mut transaction = pool.begin().await?;
    let games = get_games_by_status(&mut transaction, GameStatus::WaitingForPlayers, 10i64).await?;
    for game in games {
        let jobs = get_jobs_by_game_uuid(&mut transaction, game.id).await?;
        let start_jobs: Vec<_> = jobs
            .iter()
            .filter(|job| job.job_type == JobType::StartGame)
            .collect();
        if start_jobs.is_empty() && game.need_to_start_game() {
            Job::create_job(pool, game.id, None, JobType::StartGame).await?;
        }
    }
    transaction.commit().await?;
    Ok(ExecutionOutcome::TaskCompleted)
}
