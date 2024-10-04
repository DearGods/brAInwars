use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "update_num_participants", skip(transaction), ret, err)]
pub(crate) async fn update_num_participants(
    transaction: &mut Transaction<'_, Postgres>,
    game_id: Uuid,
    num_participants: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE games SET num_participants = $1 WHERE id = $2"#,
        num_participants,
        game_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
