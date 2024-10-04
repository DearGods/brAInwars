use crate::domain::user::User;
use crate::errors::error::Error;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "Get User opt by id",
    skip(transaction),
    level = "trace",
    ret,
    err
)]
pub(crate) async fn get_user_opt_by_id(
    transaction: &mut Transaction<'_, Postgres>,
    id: &Uuid,
) -> Result<Option<User>, Error> {
    Ok(sqlx::query_as!(
        User,
        r#"SELECT id, created_at, wallet_address, name FROM users WHERE id = $1 LIMIT 1"#,
        id
    )
    .fetch_optional(&mut **transaction)
    .await?)
}
