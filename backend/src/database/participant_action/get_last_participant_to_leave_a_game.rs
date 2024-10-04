use crate::domain::participant::participant_action::ParticipantAction;
use crate::domain::participant::participant_action_type::ParticipantActionType;
use crate::errors::error::Error;
use anyhow::anyhow;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "get last participant to leave a game",
    skip(transaction),
    ret,
    err
)]
pub(crate) async fn get_last_participant_to_leave_a_game(
    transaction: &mut Transaction<'_, Postgres>,
    game_id: Uuid,
) -> Result<Option<ParticipantAction>, Error> {
    let action = ParticipantActionType::LEAVE;
    sqlx::query_as!(
            ParticipantAction,
            r#"SELECT id, created_at, user_id, game_id, action as "action: _", signature FROM participant_actions WHERE game_id = $1 AND action = $2 ORDER BY created_at DESC LIMIT 1"#,
            game_id,
            action.to_string(),
        )
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get participant action: {:?}", e);
            Error::UnexpectedError(anyhow!("Failed to get participant action"))
        })
}
