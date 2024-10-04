use crate::domain::secret::Secret;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Nonce {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub wallet_address: String,
    #[serde(skip)]
    pub nonce: Secret<String>,
}

impl Nonce {
    pub(crate) fn generate_nonce() -> String {
        Uuid::new_v4().to_string()
    }
}
