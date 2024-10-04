use crate::domain::game::game_obj::Game;
use crate::domain::u64::U64;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "create_game", skip(transaction), ret, err)]
pub async fn create_game(
    transaction: &mut Transaction<'_, Postgres>,
    game: &Game,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO games
        (
        id,
        created_at, 
        game_id, 
        entry_fee,
        mint, 
        start_time, 
        waiting_for_players_start_time,
        winner,
        game_status, 
        wait_for_players_limit,
        players_actions, 
        hashed_limit, 
        reveled_limit,
        reveled_salt, 
        num_participants, 
        num_participants_left_game, 
        end_time, 
        mev_lock
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)"#,
        game.id,
        game.created_at,
        game.game_id.to_i64(),
        game.entry_fee.to_i64(),
        game.mint,
        U64::convert_i64_option(&game.start_time),
        U64::convert_i64_option(&game.waiting_for_players_start_time),
        game.winner,
        game.game_status.to_string(),
        game.wait_for_players_limit.to_i64(),
        game.players_actions,
        game.hashed_limit.as_ref().to_i64(),
        game.reveled_limit.as_ref().to_i64(),
        game.reveled_salt.as_ref().to_i64(),
        game.num_participants.to_i64(),
        game.num_participants_left_game.to_i64(),
        U64::convert_i64_option(&game.end_time),
        game.mev_lock
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
