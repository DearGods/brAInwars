use crate::domain::participant::participant_action::ParticipantAction;
use crate::domain::participant::participant_action_type::ParticipantActionType;
use crate::errors::error::Error;
use anyhow::anyhow;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "get participant action", skip(transaction), ret, err)]
pub(crate) async fn get_participant_action(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    game_id: Uuid,
    action: ParticipantActionType,
) -> Result<ParticipantAction, Error> {
    sqlx::query_as!(
            ParticipantAction,
            r#"SELECT id, created_at, user_id, game_id, action as "action: _", signature FROM participant_actions WHERE user_id = $1 AND game_id = $2 AND action = $3"#,
            user_id,
            game_id,
            action.to_string(),
        )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create participant action: {:?}", e);
            Error::UnexpectedError(anyhow!("Failed to create participant action"))
        })
}
