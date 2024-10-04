use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "UPDATE config value", skip(transaction), ret, err)]
pub async fn update_config_value(
    transaction: &mut Transaction<'_, Postgres>,
    key: &str,
    value: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE configs SET value = $1 WHERE key = $2"#,
        value,
        key
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
