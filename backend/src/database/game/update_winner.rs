use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "update_winner", skip(transaction), ret, err)]
pub async fn update_winner(
    transaction: &mut Transaction<'_, Postgres>,
    game_id: Uuid,
    winner: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE games SET winner = $1 WHERE id = $2"#,
        winner,
        game_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
