use crate::domain::participant::participant_obj::Participant;
use crate::domain::participant::participant_status::ParticipantStatus;
use crate::errors::error::Error;
use anyhow::anyhow;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "GET Participant", skip(transaction), ret, err)]
pub(crate) async fn get_participant(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    game_id: Uuid,
    status: ParticipantStatus,
) -> Result<Participant, Error> {
    sqlx::query_as!(
            Participant,
            r#"SELECT id, created_at, user_id, game_id, exit_time, status as "status: _" FROM participants WHERE user_id = $1 AND game_id = $2 AND status = $3 LIMIT 1"#,
            user_id,
            game_id,
            status.to_string(),
        )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get participant: {:?}", e);
            Error::UnexpectedError(anyhow!("Failed to get participant"))
        })
}
