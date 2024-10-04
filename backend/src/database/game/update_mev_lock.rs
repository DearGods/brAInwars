use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "update_mev_lock", skip(transaction), ret, err)]
pub(crate) async fn update_mev_lock(
    transaction: &mut Transaction<'_, Postgres>,
    game_id: Uuid,
    mev_lock: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE games SET mev_lock = $1 WHERE id = $2"#,
        mev_lock,
        game_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
