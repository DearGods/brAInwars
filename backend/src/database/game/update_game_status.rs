use crate::domain::game::game_status::GameStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "update_game_status", skip(transaction), ret, err)]
pub async fn update_game_status(
    transaction: &mut Transaction<'_, Postgres>,
    game_id: Uuid,
    game_status: GameStatus,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE games SET game_status = $1 WHERE id = $2"#,
        game_status.to_string(),
        game_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
