use crate::domain::participant::participant_action::ParticipantAction;
use crate::domain::participant::participant_action_type::ParticipantActionType;
use crate::errors::error::Error;
use crate::routes::games::get::PLAYER_STATUS;
use anyhow::anyhow;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "get player status", skip(transaction), ret, err)]
pub(crate) async fn get_player_status(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    game_id: Uuid,
) -> Result<PLAYER_STATUS, Error> {
    let action = sqlx::query_as!(
            ParticipantAction,
            r#"SELECT id, created_at, user_id, game_id, action as "action: _", signature FROM participant_actions WHERE user_id = $1 AND game_id = $2 ORDER BY created_at DESC LIMIT 1"#,
            user_id,
            game_id,
        )
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get player status : {:?}", e);
            Error::UnexpectedError(anyhow!("Failed to run get player status"))
        })?;
    match action {
        Some(action) => match action.action {
            ParticipantActionType::JOIN => Ok(PLAYER_STATUS::JOINED_GAME),
            ParticipantActionType::LEAVE => Ok(PLAYER_STATUS::LEFT_GAME),
            ParticipantActionType::JOINED_LEFT_BEFORE_START => Ok(PLAYER_STATUS::NOT_IN_GAME),
        },
        None => Ok(PLAYER_STATUS::NOT_IN_GAME),
    }
}
