use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "update_end_time", skip(transaction), ret, err)]
pub(crate) async fn update_game_end_time(
    transaction: &mut Transaction<'_, Postgres>,
    game_id: Uuid,
    end_time: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE games SET end_time = $1 WHERE id = $2"#,
        end_time,
        game_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
