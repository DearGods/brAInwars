use crate::blockchain::solana::game::{
    finish_game_instruction, join_game_instruction, leave_game_instruction,
    settle_game_instruction, start_game_instruction,
};
use crate::blockchain::solana::helpers::{
    create_transaction, get_account, get_game_from_blockchain, get_recent_blockhash,
    send_and_confirm_transaction,
};
use crate::blockchain::solana::pdas::{
    get_game_address, get_player_profile_address, get_player_profile_token_address,
};
use crate::blockchain::solana::player_profile::{
    init_player_profile, init_player_profile_token_account_instruction,
};
use crate::database::game::get_game::get_game;
use crate::database::game::update_game_end_time::update_game_end_time;
use crate::database::game::update_game_start_time::update_game_start_time;
use crate::database::game::update_game_status::update_game_status;
use crate::database::game::update_winner::update_winner;
use crate::database::job::create_job::create_job;
use crate::database::job::update_job_status::update_job_status;
use crate::database::job::update_job_txn::update_job_txn;
use crate::domain::game::game_status::GameStatus;
use crate::domain::id::Id;
use crate::domain::job::job_status::JobStatus;
use crate::domain::job::job_type::JobType;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::websockets::messages::{publish_msg, GameEnded, GameStarted, MsgName};
use anchor_lang::prelude::Pubkey;
use anchor_lang::solana_program::instruction::Instruction;
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use solana_sdk::signature::Signer;
use sqlx::PgPool;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Job {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub game_uuid: Uuid,
    pub wallet_address: Option<String>,
    pub job_type: JobType,
    pub status: JobStatus,
    pub error: Option<String>,
    pub txn: Option<String>,
}

impl Job {
    #[tracing::instrument(name = "Create Job", skip(pg_pool), ret, err)]
    pub async fn create_job(
        pg_pool: &PgPool,
        game_uuid: Uuid,
        wallet_address: Option<String>,
        job_type: JobType,
    ) -> anyhow::Result<()> {
        let mut transaction = pg_pool.begin().await?;
        let job = Job {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            game_uuid,
            wallet_address,
            job_type,
            status: JobStatus::Pending,
            error: None,
            txn: None,
        };
        create_job(&mut transaction, job).await?;
        transaction.commit().await?;
        Ok(())
    }

    #[tracing::instrument(name = "Run Job", skip(pg_pool, app_state), ret, err)]
    pub async fn run_job(
        &mut self,
        pg_pool: &PgPool,
        app_state: Arc<AppState>,
    ) -> anyhow::Result<()> {
        match self.job_type {
            JobType::JoinGame => self.join_game(pg_pool, app_state).await,
            JobType::LeaveGame => self.leave_game(pg_pool, app_state).await,
            JobType::FinishGame => self.finish_game(pg_pool, app_state).await,
            JobType::SettleGame => self.settle_game(pg_pool, app_state).await,
            JobType::StartGame => self.start_game(pg_pool, app_state).await,
            _ => Ok(()),
        }
    }

    #[tracing::instrument(name = "Job: Start Game", skip(pg_pool, app_state), ret, err)]
    async fn start_game(
        &mut self,
        pg_pool: &PgPool,
        app_state: Arc<AppState>,
    ) -> anyhow::Result<()> {
        // Start: Off-chain start game
        let mut transaction = pg_pool.begin().await?;
        let game = get_game(&mut transaction, self.game_uuid).await?;
        update_game_status(&mut transaction, game.id, GameStatus::OnGoing).await?;
        update_game_start_time(&mut transaction, game.id, self.created_at.timestamp()).await?;
        update_game_end_time(
            &mut transaction,
            game.id,
            (self.created_at.timestamp() as u64 + game.reveled_limit.as_ref().get()) as i64,
        )
        .await?;
        transaction.commit().await?;
        let msg = GameStarted {
            msg_name: MsgName::GAME_STARTED,
            game_id: Id::from(game.id),
            msg_id: Id::new(),
        };
        publish_msg(app_state.clone(), msg.to_string());
        // End: Off-chain start game
        // Start: On-chain start game
        let game_instructions = start_game_instruction(
            &app_state.keypair,
            game.game_id.get(),
            self.created_at.timestamp() as u64,
        )
        .map_err(Error::from)?;
        let mut transaction = pg_pool.begin().await?;
        let recent_blockhash = get_recent_blockhash(&app_state.client).await?;
        let tx = create_transaction(
            game_instructions,
            &app_state.keypair.pubkey(),
            &app_state.keypair,
            recent_blockhash,
        )
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        let signature = match send_and_confirm_transaction(&app_state.client, &tx).await {
            Ok(sig) => sig,
            Err(e) => {
                let (game_address, _) = get_game_address(game.game_id.get());
                let msg = format!(
                    "start_game job send_and_confirm_transaction: Error sending transaction: {:?} | game = {}",
                    e, game_address
                );
                tracing::error!(msg);
                update_job_status(
                    &mut transaction,
                    JobStatus::Error,
                    self.id,
                    Option::from(msg.clone()),
                )
                .await?;
                transaction.commit().await?;
                return Err(anyhow!(msg));
            }
        };
        update_job_status(&mut transaction, JobStatus::Done, self.id, None).await?;
        update_job_txn(&mut transaction, self.id, Some(signature.to_string())).await?;
        transaction.commit().await?;
        // End: On-chain start game
        Ok(())
    }

    #[tracing::instrument(name = "Job: Join Game", skip(pg_pool, app_state), ret, err)]
    async fn join_game(
        &mut self,
        pg_pool: &PgPool,
        app_state: Arc<AppState>,
    ) -> anyhow::Result<()> {
        // Start: Off-chain join game
        let mut transaction = pg_pool.begin().await?;
        if self.wallet_address.is_none() {
            let msg = "join_game job wallet address is None".to_string();
            tracing::error!(msg);
            update_job_status(
                &mut transaction,
                JobStatus::Error,
                self.id,
                Option::from(msg.clone()),
            )
            .await?;
            transaction.commit().await?;
            return Err(anyhow!(msg));
        }
        let game = get_game(&mut transaction, self.game_uuid).await?;
        let player =
            Pubkey::from_str(&self.wallet_address.clone().unwrap()).map_err(Error::from)?;
        // End: Off-chain join game
        // Start: On-chain join game
        let mint = Pubkey::from_str(&game.mint).map_err(Error::from)?;
        let instructions = join_game_instruction(
            &app_state.keypair,
            &player,
            game.game_id.get(),
            &mint,
            self.created_at.timestamp() as u64,
        )
        .map_err(Error::from)?;
        let recent_blockhash = get_recent_blockhash(&app_state.client)
            .await
            .map_err(Error::from)?;
        let tx = create_transaction(
            instructions,
            &app_state.keypair.pubkey(),
            &app_state.keypair,
            recent_blockhash,
        )
        .map_err(Error::from)?;
        let signature = match send_and_confirm_transaction(&app_state.client, &tx).await {
            Ok(sig) => sig,
            Err(e) => {
                let (game_address, _) = get_game_address(game.game_id.get());
                let msg = format!(
                    "join_game job send_and_confirm_transaction: {} | game = {} | player = {}",
                    e, game_address, player
                );
                tracing::error!(msg);
                update_job_status(
                    &mut transaction,
                    JobStatus::Error,
                    self.id,
                    Option::from(msg.clone()),
                )
                .await?;
                transaction.commit().await?;
                return Err(anyhow!(msg));
            }
        };
        update_job_txn(&mut transaction, self.id, Some(signature.to_string())).await?;
        update_job_status(&mut transaction, JobStatus::Done, self.id, None).await?;
        transaction.commit().await?;
        // End: On-chain join game
        Ok(())
    }

    #[tracing::instrument(name = "Job: Leave Game", skip(pg_pool, app_state), ret, err)]
    async fn leave_game(
        &mut self,
        pg_pool: &PgPool,
        app_state: Arc<AppState>,
    ) -> anyhow::Result<()> {
        // Start: Off-chain leave game
        let mut transaction = pg_pool.begin().await?;
        if self.wallet_address.is_none() {
            let msg = "leave_game job wallet address is None".to_string();
            tracing::error!(msg);
            update_job_status(
                &mut transaction,
                JobStatus::Error,
                self.id,
                Option::from(msg.clone()),
            )
            .await?;
            transaction.commit().await?;
            return Err(anyhow!(msg));
        }
        let game = get_game(&mut transaction, self.game_uuid).await?;
        let player =
            Pubkey::from_str(&self.wallet_address.clone().unwrap()).map_err(Error::from)?;
        let mint = Pubkey::from_str(&game.mint).map_err(Error::from)?;
        // End: Off-chain leave game
        // Start: On-chain leave game
        let leave_game_instructions = leave_game_instruction(
            &app_state.keypair,
            &player,
            game.game_id.get(),
            &mint,
            self.created_at.timestamp() as u64,
        )
        .map_err(Error::from)?;
        let recent_blockhash = get_recent_blockhash(&app_state.client)
            .await
            .map_err(Error::from)?;
        let tx = create_transaction(
            leave_game_instructions,
            &app_state.keypair.pubkey(),
            &app_state.keypair,
            recent_blockhash,
        )
        .map_err(Error::from)?;
        let signature = match send_and_confirm_transaction(&app_state.client, &tx).await {
            Ok(sig) => sig,
            Err(e) => {
                let (game_address, _) = get_game_address(game.game_id.get());
                let msg = format!(
                    "leave_game job send_and_confirm_transaction: {} | game = {} | player = {}",
                    e, game_address, player
                );
                tracing::error!(msg);
                update_job_status(
                    &mut transaction,
                    JobStatus::Error,
                    self.id,
                    Option::from(msg.clone()),
                )
                .await?;
                transaction.commit().await?;
                return Err(anyhow!(msg));
            }
        };
        update_job_status(&mut transaction, JobStatus::Done, self.id, None).await?;
        update_job_txn(&mut transaction, self.id, Some(signature.to_string())).await?;
        transaction.commit().await?;
        // End: On-chain leave game
        Ok(())
    }

    #[tracing::instrument(name = "Job: Finish Game", skip(pg_pool, app_state), ret, err)]
    async fn finish_game(
        &mut self,
        pg_pool: &PgPool,
        app_state: Arc<AppState>,
    ) -> anyhow::Result<()> {
        // Start: Off-chain finish game
        let mut transaction = pg_pool.begin().await?;
        let game = get_game(&mut transaction, self.game_uuid).await?;
        game.finish_game(pg_pool, &app_state.keypair.pubkey())
            .await?;
        transaction.commit().await?;
        // End: Off-chain finish game
        // Start: On-chain finish game
        let mut transaction = pg_pool.begin().await?;
        let game = get_game(&mut transaction, self.game_uuid).await?;
        let winner = game.winner.unwrap();
        let msg = GameEnded {
            msg_name: MsgName::GAME_ENDED,
            game_id: Id::from(game.id),
            msg_id: Id::new(),
            winner: if winner == app_state.keypair.pubkey().to_string() {
                None
            } else {
                Some(winner.clone())
            },
        };
        publish_msg(app_state.clone(), msg.to_string());
        let winner = Pubkey::from_str(&winner).map_err(Error::from)?;
        let game_instructions = finish_game_instruction(
            &app_state.keypair,
            game.game_id.get(),
            game.reveled_limit.as_ref().get(),
            game.reveled_salt.as_ref().get(),
            self.created_at.timestamp() as u64,
            &winner,
        )
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        let mut transaction = pg_pool.begin().await?;
        let recent_blockhash = get_recent_blockhash(&app_state.client).await?;
        let tx = create_transaction(
            game_instructions,
            &app_state.keypair.pubkey(),
            &app_state.keypair,
            recent_blockhash,
        )
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        let signature = match send_and_confirm_transaction(&app_state.client, &tx).await {
            Ok(sig) => sig,
            Err(e) => {
                let (game_address, _) = get_game_address(game.game_id.get());
                let msg = format!(
                    "finish_game job send_and_confirm_transaction: {} | game = {} | winner = {}",
                    e, game_address, winner
                );
                tracing::error!(msg);
                update_job_status(
                    &mut transaction,
                    JobStatus::Error,
                    self.id,
                    Option::from(msg.clone()),
                )
                .await?;
                transaction.commit().await?;
                return Err(anyhow!(msg));
            }
        };
        update_job_status(&mut transaction, JobStatus::Done, self.id, None).await?;
        update_job_txn(&mut transaction, self.id, Some(signature.to_string())).await?;
        Job::create_job(
            pg_pool,
            self.game_uuid,
            Some(winner.to_string()),
            JobType::SettleGame,
        )
        .await?;
        transaction.commit().await?;
        Ok(())
    }

    #[tracing::instrument(name = "Job: Settle Game", skip(pg_pool, app_state), ret, err)]
    async fn settle_game(
        &mut self,
        pg_pool: &PgPool,
        app_state: Arc<AppState>,
    ) -> anyhow::Result<()> {
        let mut transaction = pg_pool.begin().await?;
        let game = get_game(&mut transaction, self.game_uuid).await?;
        let winner = game.winner.unwrap();
        let on_chain_game =
            get_game_from_blockchain(app_state.clone(), game.game_id.get(), game.id).await?;
        if game.game_status == GameStatus::Settled {
            update_job_status(&mut transaction, JobStatus::Done, self.id, None).await?;
            transaction.commit().await?;
            return Ok(());
        }
        if game.game_status != GameStatus::Finished {
            let msg = format!("settle_game job game is not finished: {:?}", on_chain_game);
            tracing::error!(msg);
            update_job_status(
                &mut transaction,
                JobStatus::Pending,
                self.id,
                Option::from(msg.clone()),
            )
            .await?;
            transaction.commit().await?;
            return Err(anyhow!(msg));
        }
        let winner = Pubkey::from_str(&winner).unwrap();
        update_winner(&mut transaction, game.id, &winner.to_string()).await?;
        let mut instructions: Vec<Instruction> = vec![];
        let mint = Pubkey::from_str(&game.mint).map_err(|e| anyhow::Error::msg(e.to_string()))?;
        let (signer_player_profile, _) = get_player_profile_address(&app_state.keypair.pubkey());
        let signer_player_profile_exists = get_account(&app_state.client, &signer_player_profile)
            .await
            .map_err(Error::from)?;
        if signer_player_profile_exists.is_none() {
            let inst = init_player_profile(&app_state.keypair, &mint).map_err(Error::from)?;
            instructions.extend(inst);
        }
        let (signer_player_token_profile, _) =
            get_player_profile_token_address(&app_state.keypair.pubkey(), &mint);
        let signer_player_token_profile_exists =
            get_account(&app_state.client, &signer_player_token_profile)
                .await
                .map_err(Error::from)?;
        if signer_player_token_profile_exists.is_none() {
            let inst = init_player_profile_token_account_instruction(&app_state.keypair, &mint)
                .map_err(Error::from)?;
            instructions.extend(inst);
        }
        let settle_game_instructions = settle_game_instruction(
            &app_state.keypair,
            &winner,
            game.game_id.get(),
            &mint,
            self.created_at.timestamp() as u64,
        )
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        instructions.extend(settle_game_instructions);
        let recent_blockhash = get_recent_blockhash(&app_state.client).await?;
        let tx = create_transaction(
            instructions,
            &app_state.keypair.pubkey(),
            &app_state.keypair,
            recent_blockhash,
        )
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        let signature = match send_and_confirm_transaction(&app_state.client, &tx).await {
            Ok(sig) => sig,
            Err(e) => {
                let (game_address, _) = get_game_address(game.game_id.get());
                let msg = format!(
                    "settle_game job send_and_confirm_transaction: {} | game = {} | winner = {}",
                    e, game_address, winner
                );
                tracing::error!(msg);
                update_job_status(
                    &mut transaction,
                    JobStatus::Error,
                    self.id,
                    Option::from(msg.clone()),
                )
                .await?;
                transaction.commit().await?;
                return Err(anyhow!(msg));
            }
        };
        update_game_status(&mut transaction, game.id, GameStatus::Settled).await?;
        update_job_status(&mut transaction, JobStatus::Done, self.id, None).await?;
        update_job_txn(&mut transaction, self.id, Some(signature.to_string())).await?;
        transaction.commit().await?;
        let msg = GameEnded {
            msg_name: MsgName::GAME_SETTLED,
            game_id: Id::from(game.id),
            msg_id: Id::new(),
            winner: if winner == app_state.keypair.pubkey() {
                None
            } else {
                Some(winner.to_string())
            },
        };
        publish_msg(app_state.clone(), msg.to_string());
        Ok(())
    }
}
