use crate::errors::error::Error;
use chrono::{DateTime, Utc};
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub wallet_address: String,
    pub name: String,
}

impl User {
    #[tracing::instrument(name = "New User", ret, err)]
    pub async fn new(wallet_address: &str) -> Result<User, Error> {
        let id = Uuid::new_v4();
        let short_id = &id.to_string()[0..8];
        let created_at = Utc::now();
        let name: String = Name(EN).fake();
        let name = format!("{}_{}", name, short_id);

        Ok(User {
            id,
            created_at,
            wallet_address: wallet_address.to_string(),
            name,
        })
    }
}
