use crate::domain::participant::participant_obj::Participant;
use crate::errors::error::Error;
use anyhow::anyhow;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "GET All Game Participants", skip(transaction), ret, err)]
pub(crate) async fn get_all_game_participants(
    transaction: &mut Transaction<'_, Postgres>,
    game_id: Uuid,
) -> Result<Vec<Participant>, Error> {
    sqlx::query_as!(
            Participant,
            r#"SELECT id, created_at, user_id, game_id, exit_time, status as "status: _" FROM participants WHERE game_id = $1"#,
            game_id,
        )
        .fetch_all(&mut **transaction)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get all game participants: {:?}", e);
            Error::UnexpectedError(anyhow!("Failed to get all game participants"))
        })
}
