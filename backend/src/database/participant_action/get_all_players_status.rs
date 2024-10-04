use crate::domain::date::Date;
use crate::domain::participant::participant_action_type::ParticipantActionType;
use crate::errors::error::Error;
use crate::routes::games::get::PLAYER_STATUS;
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use typeshare::typeshare;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct PreProcessedGamePlayersStatus {
    pub wallet_address: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub action: ParticipantActionType,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct GamePlayersStatus {
    pub wallet_address: String,
    pub name: String,
    pub created_at: Date,
    pub player_status: PLAYER_STATUS,
}

#[tracing::instrument(name = "get all players status", skip(transaction), ret, err)]
pub(crate) async fn get_all_players_status(
    transaction: &mut Transaction<'_, Postgres>,
    game_id: Uuid,
) -> Result<Vec<GamePlayersStatus>, Error> {
    let actions = sqlx::query_as!(
            PreProcessedGamePlayersStatus,
            r#"SELECT DISTINCT ON (participant_actions.user_id) users.wallet_address, users.name, participant_actions.created_at, action as "action: _" FROM participant_actions JOIN users ON participant_actions.user_id = users.id WHERE participant_actions.game_id = $1 ORDER BY participant_actions.user_id, participant_actions.created_at DESC"#,
            game_id,
        )
        .fetch_all(&mut **transaction)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get player status : {:?}", e);
            Error::UnexpectedError(anyhow!("Failed to run get player status"))
        })?;

    Ok(actions
        .into_iter()
        .map(|action| {
            let player_status = match action.action {
                ParticipantActionType::JOIN => PLAYER_STATUS::JOINED_GAME,
                ParticipantActionType::LEAVE => PLAYER_STATUS::LEFT_GAME,
                ParticipantActionType::JOINED_LEFT_BEFORE_START => PLAYER_STATUS::NOT_IN_GAME,
            };

            GamePlayersStatus {
                name: action.name,
                wallet_address: action.wallet_address,
                created_at: Date::from(action.created_at),
                player_status,
            }
        })
        .collect())
}
