use crate::domain::u64::U64;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "update_num_participants_left", skip(transaction), ret, err)]
pub(crate) async fn update_num_participants_left(
    transaction: &mut Transaction<'_, Postgres>,
    game_id: Uuid,
    num_participants_left_game: U64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE games SET num_participants_left_game = $1 WHERE id = $2"#,
        num_participants_left_game.to_i64(),
        game_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
