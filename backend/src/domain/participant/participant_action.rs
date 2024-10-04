use crate::domain::participant::participant_action_type::ParticipantActionType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct ParticipantAction {
    pub id: Uuid,
    pub game_id: Uuid,
    pub user_id: Uuid,
    pub action: ParticipantActionType,
    pub created_at: DateTime<Utc>,
    pub signature: String,
}
