use crate::database::user::get_user_opt_by_wallet::get_user_opt_by_wallet;
use crate::domain::user::User;
use crate::errors::error::Error;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "Get User by wallet", skip(transaction), ret, err)]
pub async fn get_user_by_wallet(
    transaction: &mut Transaction<'_, Postgres>,
    wallet_address: &str,
) -> Result<User, Error> {
    match get_user_opt_by_wallet(transaction, wallet_address).await? {
        None => Err(Error::UserNotFound),
        Some(i) => Ok(i),
    }
}
