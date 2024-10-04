use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "update_user_name", skip(transaction), ret, err)]
pub(crate) async fn update_user_name(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    name: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"UPDATE users SET name = $1 WHERE id = $2"#, name, user_id)
        .execute(&mut **transaction)
        .await?;
    Ok(())
}
