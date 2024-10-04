use crate::domain::config::Config;
use crate::domain::u64::U64;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::workers::tasks::create_games_for_entries::create_game_for_entries;
use crate::workers::tasks::delete_nonces::delete_nonces;
use crate::workers::tasks::finish_games::finish_games;
use crate::workers::tasks::start_games::start_games;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use std::time::Duration;

pub type PgTransaction = Transaction<'static, Postgres>;

#[derive(Debug)]
pub enum ExecutionOutcome {
    TaskCompleted,
    EmptyQueue,
}

pub async fn worker_loop(pool: PgPool, app_state: Arc<AppState>) -> Result<(), anyhow::Error> {
    let configs = Config::get_configs_hashmap(&pool)
        .await
        .map_err(Error::from)?;
    let countdown_for_others_to_join =
        Config::get_config_value(&configs, "countdown_for_others_to_join")?
            .as_u64()
            .ok_or(Error::FailedToCastConfigToKeyValue(
                "countdown_for_others_to_join".to_string(),
            ))?;
    let worker_interval = Config::get_config_value(&configs, "worker_interval")?
        .as_u64()
        .ok_or(Error::FailedToCastConfigToKeyValue(
            "worker_interval".to_string(),
        ))?;

    let min_game_length = Config::get_config_value(&configs, "min_game_length")?
        .as_u64()
        .ok_or(Error::FailedToCastConfigToKeyValue(
            "min_game_length".to_string(),
        ))?;
    let max_game_length = Config::get_config_value(&configs, "max_game_length")?
        .as_u64()
        .ok_or(Error::FailedToCastConfigToKeyValue(
            "max_game_length".to_string(),
        ))?;

    let game_entries = Config::get_config_value(&configs, "game_entries")?
        .as_array()
        .ok_or(Error::FailedToCastConfigToKeyValue(
            "game_entries".to_string(),
        ))?
        .iter()
        .filter_map(|v| v.as_f64())
        .collect::<Vec<f64>>();
    loop {
        match delete_nonces(&pool).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("worker_loop: delete_nonce_by_date: error: {}", e);
                tokio::time::sleep(Duration::from_millis(worker_interval)).await;
            }
        }
        match create_game_for_entries(
            &pool,
            app_state.clone(),
            &game_entries,
            U64::from(min_game_length),
            U64::from(max_game_length),
            U64::from(countdown_for_others_to_join),
        )
        .await
        {
            Ok(ExecutionOutcome::EmptyQueue) => {}
            Ok(ExecutionOutcome::TaskCompleted) => {}
            Err(e) => {
                tracing::error!("worker_loop: create_game_for_entries: error: {}", e);
                tokio::time::sleep(Duration::from_millis(worker_interval)).await;
            }
        }
        match start_games(&pool, app_state.clone()).await {
            Ok(ExecutionOutcome::EmptyQueue) => {}
            Ok(ExecutionOutcome::TaskCompleted) => {}
            Err(e) => {
                tracing::error!("worker_loop: start_games: error: {}", e);
                tokio::time::sleep(Duration::from_millis(worker_interval)).await;
            }
        }
        match finish_games(&pool, app_state.clone()).await {
            Ok(ExecutionOutcome::EmptyQueue) => {}
            Ok(ExecutionOutcome::TaskCompleted) => {}
            Err(e) => {
                tracing::error!("worker_loop: finish_games: error: {}", e);
                tokio::time::sleep(Duration::from_millis(worker_interval)).await;
            }
        }
        tokio::time::sleep(Duration::from_millis(worker_interval)).await;
    }
}
