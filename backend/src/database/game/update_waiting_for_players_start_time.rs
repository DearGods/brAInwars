use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "update_waiting_for_players_start_time",
    skip(transaction),
    ret,
    err
)]
pub(crate) async fn update_waiting_for_players_start_time(
    transaction: &mut Transaction<'_, Postgres>,
    game_id: Uuid,
    waiting_for_players_start_time: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE games SET waiting_for_players_start_time = $1 WHERE id = $2"#,
        waiting_for_players_start_time,
        game_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
