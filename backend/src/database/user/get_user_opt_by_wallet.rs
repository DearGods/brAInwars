use crate::domain::user::User;
use crate::errors::error::Error;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "Get User opt by wallet", skip(transaction), ret, err)]
pub async fn get_user_opt_by_wallet(
    transaction: &mut Transaction<'_, Postgres>,
    wallet_address: &str,
) -> Result<Option<User>, Error> {
    Ok(sqlx::query_as!(
        User,
        r#"SELECT id, created_at, wallet_address, name FROM users WHERE wallet_address = $1 LIMIT 1"#,
        wallet_address
    )
        .fetch_optional(&mut **transaction)
        .await?)
}
