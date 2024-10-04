use crate::database::game::get_expired_in_progress_games::get_expired_in_progress_games;
use crate::database::game::get_games_all_participants_left::get_games_all_participants_left;
use crate::database::game::get_mev_locked_ongoing::get_mev_locked_ongoing;
use crate::database::game::update_mev_lock::update_mev_lock;
use crate::database::job::get_jobs_by_id::get_jobs_by_game_uuid;
use crate::domain::job::job_obj::Job;
use crate::domain::job::job_type::JobType;
use crate::startup::application::AppState;
use crate::workers::game_worker::ExecutionOutcome;
use sqlx::PgPool;
use std::sync::Arc;

#[tracing::instrument(name = "finish_game", level = "trace", skip(pool, _app_state), err)]
pub async fn finish_games(
    pool: &PgPool,
    _app_state: Arc<AppState>,
) -> Result<ExecutionOutcome, anyhow::Error> {
    let mut games = Vec::new();
    let mut transaction = pool.begin().await?;
    for game in get_mev_locked_ongoing(&mut transaction, 10i64).await? {
        games.push(game);
    }
    for game in get_expired_in_progress_games(&mut transaction, 10i64).await? {
        games.push(game);
    }
    for game in get_games_all_participants_left(&mut transaction, 10i64).await? {
        games.push(game);
    }
    for game in games.iter() {
        update_mev_lock(&mut transaction, game.id, true).await?;
    }
    transaction.commit().await?;

    let mut transaction = pool.begin().await?;
    for game in games {
        let jobs = get_jobs_by_game_uuid(&mut transaction, game.id).await?;
        let finish_jobs: Vec<_> = jobs
            .iter()
            .filter(|job| job.job_type == JobType::FinishGame)
            .collect();
        if finish_jobs.is_empty() {
            Job::create_job(pool, game.id, None, JobType::FinishGame).await?;
        }
    }
    transaction.commit().await?;

    Ok(ExecutionOutcome::TaskCompleted)
}
