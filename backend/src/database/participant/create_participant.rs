use crate::domain::participant::participant_status::ParticipantStatus;
use crate::errors::error::Error;
use anyhow::anyhow;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Create Participant", skip(transaction), ret, err)]
pub(crate) async fn create_participant(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    game_id: Uuid,
) -> Result<(), Error> {
    // strange unused import false warning
    let status = ParticipantStatus::JOINED.to_string();
    let now = Utc::now();
    sqlx::query!(
            r#"INSERT INTO participants (id, created_at, user_id, game_id, status) VALUES ($1, $2, $3, $4, $5)"#,
            Uuid::new_v4(),
            now,
            user_id,
            game_id,
            status,
        )
        .execute(&mut **transaction)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create participant: {:?}", e);
            Error::UnexpectedError(anyhow!("Failed to create participant"))
        })?;
    Ok(())
}
