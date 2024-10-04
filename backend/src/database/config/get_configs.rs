use crate::domain::config::Config;
use sqlx::PgPool;

#[tracing::instrument(name = "Get game by UUID", skip(pool), ret, err)]
pub async fn get_configs(pool: &PgPool) -> Result<Vec<Config>, sqlx::Error> {
    let configs = sqlx::query_as!(Config, r#"SELECT id,key,value,created_at FROM configs"#,)
        .fetch_all(pool)
        .await?;
    Ok(configs)
}
