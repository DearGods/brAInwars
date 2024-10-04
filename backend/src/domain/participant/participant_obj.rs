use crate::domain::participant::participant_status::ParticipantStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Participant {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub game_id: Uuid,
    pub user_id: Uuid,
    pub exit_time: Option<DateTime<Utc>>,
    pub status: ParticipantStatus,
}
