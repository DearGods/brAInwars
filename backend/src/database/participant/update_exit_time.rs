use chrono::{DateTime, Utc};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "UPDATE exit time", skip(transaction), ret, err)]
pub(crate) async fn update_exit_time(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    game_id: Uuid,
    exit_time: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE participants SET exit_time = $1 WHERE user_id = $2 AND game_id = $3"#,
        exit_time,
        user_id,
        game_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
