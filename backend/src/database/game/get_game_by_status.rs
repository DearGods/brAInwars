use crate::domain::game::game_obj::Game;
use crate::domain::game::game_status::GameStatus;
use crate::domain::secret::Secret;
use crate::domain::u64::U64;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(
    name = "get_game_by_status",
    level = "trace",
    skip(transaction),
    ret,
    err
)]
pub async fn get_game_by_status(
    transaction: &mut Transaction<'_, Postgres>,
    game_status: GameStatus,
) -> Result<Option<Game>, sqlx::Error> {
    let game = sqlx::query_as!(
        Game,
        r#"SELECT
        id,
        created_at,
        mint,
        winner,
        game_id as "game_id: U64",
        entry_fee as "entry_fee: U64",
        start_time as "start_time: U64",
        waiting_for_players_start_time as "waiting_for_players_start_time: U64",
        game_status as "game_status: GameStatus",
        wait_for_players_limit as "wait_for_players_limit: U64",
        players_actions,
        hashed_limit as "hashed_limit: Secret<U64>",
        reveled_limit as "reveled_limit: Secret<U64>",
        reveled_salt as "reveled_salt: Secret<U64>",
        num_participants as "num_participants: U64",
        num_participants_left_game as "num_participants_left_game: U64",
        end_time as "end_time: U64",
        mev_lock
        FROM games
        WHERE game_status = $1
        ORDER BY end_time ASC
        LIMIT 1"#,
        game_status.to_string()
    )
    .fetch_optional(&mut **transaction)
    .await?;
    Ok(game)
}
