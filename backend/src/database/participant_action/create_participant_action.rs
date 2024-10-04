use crate::domain::participant::participant_action_type::ParticipantActionType;
use crate::errors::error::Error;
use anyhow::anyhow;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "create Participant action", skip(transaction), ret, err)]
pub(crate) async fn create_participant_action(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    game_id: Uuid,
    action: ParticipantActionType,
    signature: String,
) -> Result<(), Error> {
    let now = Utc::now();
    sqlx::query!(
            r#"INSERT INTO participant_actions (id, created_at, user_id, game_id, action, signature) VALUES ($1, $2, $3, $4, $5, $6)"#,
            Uuid::new_v4(),
            now,
            user_id,
            game_id,
            action.to_string(),
            signature
        )
        .execute(&mut **transaction)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create participant action: {:?}", e);
            Error::UnexpectedError(anyhow!("Failed to create participant action"))
        })?;
    Ok(())
}
