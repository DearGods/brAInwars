use crate::domain::game::game_status::GameStatus;
use crate::errors::error::Error;
use anchor_lang::prelude::Pubkey;
use anchor_lang::solana_program::keccak::hash;
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::blockchain::solana::pdas::{get_game_address, get_players_actions_address};
use crate::database::game::update_game_status::update_game_status;
use crate::database::game::update_winner::update_winner;
use crate::database::participant_action::get_last_participant_to_leave_a_game::get_last_participant_to_leave_a_game;
use crate::database::user::get_user_opt_by_id::get_user_opt_by_id;
use crate::domain::secret::Secret;
use crate::domain::u64::U64;
use uuid::Uuid;

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Game {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub game_id: U64,
    pub entry_fee: U64,
    pub mint: String,
    pub start_time: Option<U64>,
    pub waiting_for_players_start_time: Option<U64>,
    pub winner: Option<String>,
    pub game_status: GameStatus,
    pub wait_for_players_limit: U64,
    pub players_actions: String,
    #[serde(skip)]
    pub hashed_limit: Secret<U64>,
    #[serde(skip)]
    pub reveled_limit: Secret<U64>,
    #[serde(skip)]
    pub reveled_salt: Secret<U64>,
    pub end_time: Option<U64>,
    pub num_participants: U64,
    pub num_participants_left_game: U64,
    pub mev_lock: bool,
}

impl Game {
    #[tracing::instrument(name = "game_obj::finish_game", skip(pg_pool, signer), ret, err)]
    pub async fn finish_game(&self, pg_pool: &PgPool, signer: &Pubkey) -> Result<(), Error> {
        let mut transaction = pg_pool.begin().await?;
        if self.winner.is_some()
            || self.game_status == GameStatus::Finished
            || self.game_status == GameStatus::Settled
        {
            return Ok(());
        }
        let winner = get_last_participant_to_leave_a_game(&mut transaction, self.id).await?;
        match winner {
            Some(winner) => {
                let user = get_user_opt_by_id(&mut transaction, &winner.user_id).await?;
                if let Some(user) = user {
                    update_winner(&mut transaction, self.id, &user.wallet_address).await?;
                }
            }
            None => {
                update_winner(&mut transaction, self.id, &signer.to_string()).await?;
            }
        }
        update_game_status(&mut transaction, self.id, GameStatus::Finished).await?;
        transaction.commit().await?;
        Ok(())
    }

    #[tracing::instrument(name = "validate GameLength", ret, err)]
    pub fn validate_game_length(
        game_length: U64,
        min_game_length: U64,
        max_game_length: U64,
    ) -> Result<Secret<U64>, anyhow::Error> {
        if game_length.get() > max_game_length.get() {
            return Err(Error::GameLongerThanAllowed.into());
        } else if game_length.get() < min_game_length.get() {
            return Err(Error::GameShorterThanAllowed.into());
        }
        Ok(Secret::from(game_length))
    }

    pub fn need_to_start_game(&self) -> bool {
        let time = chrono::Utc::now();
        if self.waiting_for_players_start_time.is_none() {
            return false;
        }
        time.timestamp()
            >= self.waiting_for_players_start_time.unwrap().to_i64()
                + self.wait_for_players_limit.to_i64()
    }

    #[tracing::instrument(name = "new Game", ret, err)]
    pub fn new(
        entry_fee: U64,
        mint: String,
        min_game_length: U64,
        max_game_length: U64,
        wait_for_players_limit: U64,
    ) -> Result<Game, Error> {
        let id = Uuid::new_v4();
        let game_id = U64::from(rand::thread_rng().gen_range(0u64..1_000_000_000u64));
        let reveled_salt = Secret::from(U64::from(
            rand::thread_rng().gen_range(0u64..1_000_000_000u64),
        ));
        let reveled_limit = Game::validate_game_length(
            U64::from(rand::thread_rng().gen_range(min_game_length.get()..max_game_length.get())),
            min_game_length,
            max_game_length,
        )?;
        let (game_key, _) = get_game_address(game_id.get());
        let (players_action, _) = get_players_actions_address(game_id.get());
        let hash_input = format!(
            "{}{}{}",
            game_key,
            reveled_limit.as_ref().get(),
            reveled_salt.as_ref().get()
        );

        let h = hash(hash_input.as_bytes());
        let mut hashed_limit = 0u64;
        h.0.iter().for_each(|byte| {
            hashed_limit += *byte as u64;
        });

        Ok(Game {
            id,
            created_at: Utc::now(),
            game_id,
            mint,
            entry_fee,
            start_time: None,
            waiting_for_players_start_time: None,
            winner: None,
            game_status: GameStatus::WaitingForPlayers,
            wait_for_players_limit,
            players_actions: players_action.to_string(),
            reveled_limit,
            reveled_salt,
            hashed_limit: Secret::from(U64::from(hashed_limit)),
            end_time: None,
            num_participants: U64::from(0),
            num_participants_left_game: U64::from(0),
            mev_lock: false,
        })
    }
}
