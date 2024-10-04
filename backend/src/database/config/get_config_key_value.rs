use crate::domain::config::Config;
use sqlx::PgPool;

#[tracing::instrument(name = "get_config_key_value", skip(pool), ret, err)]
pub async fn get_config_key_value(pool: &PgPool, key: &str) -> Result<Config, sqlx::Error> {
    let config = sqlx::query_as!(
        Config,
        r#"SELECT id,key,value,created_at FROM configs WHERE key = $1 LIMIT 1"#,
        key
    )
    .fetch_one(pool)
    .await?;
    Ok(config)
}
