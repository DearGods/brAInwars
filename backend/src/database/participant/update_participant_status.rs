use crate::domain::participant::participant_status::ParticipantStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "UPDATE participant status", skip(transaction), ret, err)]
pub(crate) async fn update_participant_status(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    game_id: Uuid,
    status: ParticipantStatus,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE participants SET status = $1 WHERE user_id = $2 AND game_id = $3"#,
        status.to_string(),
        user_id,
        game_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
