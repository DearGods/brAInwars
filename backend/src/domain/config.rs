use crate::database::config::get_configs::get_configs;
use crate::errors::error::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub id: Uuid,
    pub key: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
}

impl TryInto<serde_json::Value> for Config {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<serde_json::Value, Self::Error> {
        serde_json::from_str(&self.value)
    }
}

impl Config {
    pub async fn get_configs_hashmap(
        pool: &PgPool,
    ) -> Result<HashMap<String, serde_json::Value>, sqlx::Error> {
        let configs = get_configs(pool).await?;
        let configs_hashmap = configs
            .iter()
            .map(|config| (config.key.clone(), serde_json::from_str(&config.value)))
            .filter_map(|(key, value)| match value {
                Ok(value) => Some((key, value)),
                Err(_) => None,
            })
            .collect::<HashMap<_, _>>();
        Ok(configs_hashmap)
    }

    pub fn get_config_value<'a>(
        configs: &'a HashMap<String, serde_json::Value>,
        key: &str,
    ) -> Result<&'a serde_json::Value, Error> {
        configs
            .get(key)
            .ok_or(Error::ConfigNotFound(key.to_string()))?
            .get(key)
            .ok_or(Error::ConfigNotFound(key.to_string()))
    }
}
