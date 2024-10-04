use crate::blockchain::solana::crank::init_crank_token_instruction;
use crate::blockchain::solana::game::create_game_instruction;
use crate::blockchain::solana::helpers::{
    create_transaction, get_account, get_recent_blockhash, send_and_confirm_transaction,
};
use crate::blockchain::solana::pdas::get_crank_token_address;
use crate::database::game::create_game::create_game;
use crate::database::game::get_game_by_status_and_entry::get_game_by_status_and_entry;
use crate::domain::game::game_obj::Game;
use crate::domain::game::game_status::GameStatus;
use crate::domain::id::Id;
use crate::domain::u64::U64;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::websockets::messages::{publish_msg, MsgName, NewGameCreated};
use crate::workers::game_worker::ExecutionOutcome;
use anchor_lang::prelude::Pubkey;
use anchor_spl::token::spl_token;
use anchor_spl::token_interface::spl_token_2022::solana_program::instruction::Instruction;
use chrono::Utc;
use solana_sdk::signature::Signer;
use sqlx::{PgPool, Postgres, Transaction};
use std::str::FromStr;
use std::sync::Arc;

#[tracing::instrument(
    name = "create_game_for_entries_db",
    level = "trace",
    skip(transaction),
    err
)]
pub async fn create_game_for_entries_db(
    transaction: &mut Transaction<'_, Postgres>,
    game_entries: &Vec<f64>,
    min_game_length: U64,
    max_game_length: U64,
    games: &mut Vec<Game>,
    countdown_for_others_to_join: U64,
) -> Result<ExecutionOutcome, anyhow::Error> {
    for entry in game_entries.iter() {
        let entry_fee = U64::new(*entry as i64)?;
        let not_started = get_game_by_status_and_entry(
            &mut *transaction,
            entry_fee,
            GameStatus::WaitingForPlayers,
        )
        .await?;
        let in_progress =
            get_game_by_status_and_entry(&mut *transaction, entry_fee, GameStatus::OnGoing).await?;
        if not_started.is_none() && in_progress.is_none() {
            let game = Game::new(
                entry_fee,
                spl_token::native_mint::id().to_string(),
                // USDC.to_string()
                min_game_length,
                max_game_length,
                countdown_for_others_to_join,
            )?;
            create_game(&mut *transaction, &game).await?;
            games.push(game);
        }
    }
    Ok(ExecutionOutcome::TaskCompleted)
}

#[tracing::instrument(
    name = "create_game_for_entries_blockchain",
    level = "trace",
    skip(app_state),
    err
)]
pub async fn create_game_for_entries_blockchain(
    app_state: Arc<AppState>,
    games: &mut Vec<Game>,
) -> Result<ExecutionOutcome, anyhow::Error> {
    for game in games {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mint = Pubkey::from_str(&game.mint).map_err(Error::from)?;
        let (crank_token_account, _) = get_crank_token_address(&mint);
        let crank_token_account_exists = get_account(&app_state.client, &crank_token_account)
            .await
            .map_err(Error::from)?;
        if crank_token_account_exists.is_none() {
            instructions.extend(
                init_crank_token_instruction(&app_state.keypair, &mint)
                    .await
                    .map_err(Error::from)?,
            );
        }
        let game_instructions = create_game_instruction(
            &app_state.keypair,
            game.game_id.get(),
            game.wait_for_players_limit.get(),
            game.hashed_limit.as_ref().get(),
            game.entry_fee.get(),
            &Pubkey::from_str(&game.mint).map_err(Error::from)?,
            Utc::now().timestamp() as u64,
        )
        .map_err(Error::from)?;
        instructions.extend(game_instructions);
        let recent_blockhash = get_recent_blockhash(&app_state.client).await?;
        let tx = create_transaction(
            instructions,
            &app_state.keypair.pubkey(),
            &app_state.keypair,
            recent_blockhash,
        )
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        let signature = send_and_confirm_transaction(&app_state.client, &tx)
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        tracing::info!(
            "send_and_confirm_transaction => create_game_for_entries_blockchain signature: {:?} | recent_blockhash {:?} | game_id {:?} | game_uuid {:?}",
            signature,
            recent_blockhash,
            game.game_id.get(),
            game.id
        );
        let msg = NewGameCreated {
            msg_name: MsgName::NEW_GAME_CREATED,
            game_id: Id::from(game.id),
            msg_id: Id::new(),
        };
        publish_msg(app_state.clone(), msg.to_string());
    }
    Ok(ExecutionOutcome::TaskCompleted)
}

#[tracing::instrument(
    name = "create_game_for_entries_publish",
    level = "trace",
    skip(app_state),
    err
)]
pub async fn create_game_for_entries_publish(
    app_state: Arc<AppState>,
    games: &mut Vec<Game>,
) -> Result<ExecutionOutcome, anyhow::Error> {
    for game in games {
        let msg = NewGameCreated {
            msg_name: MsgName::NEW_GAME_CREATED,
            game_id: Id::from(game.id),
            msg_id: Id::new(),
        };
        publish_msg(app_state.clone(), msg.to_string());
    }
    Ok(ExecutionOutcome::TaskCompleted)
}

#[tracing::instrument(
    name = "create_game_for_entry",
    level = "trace",
    skip(pool, app_state),
    err
)]
pub async fn create_game_for_entries(
    pool: &PgPool,
    app_state: Arc<AppState>,
    game_entries: &Vec<f64>,
    min_game_length: U64,
    max_game_length: U64,
    countdown_for_others_to_join: U64,
) -> Result<ExecutionOutcome, anyhow::Error> {
    let mut transaction = pool.begin().await?;
    let mut games = Vec::new();
    create_game_for_entries_db(
        &mut transaction,
        game_entries,
        min_game_length,
        max_game_length,
        &mut games,
        countdown_for_others_to_join,
    )
    .await?;
    create_game_for_entries_blockchain(app_state.clone(), &mut games).await?;
    create_game_for_entries_publish(app_state.clone(), &mut games).await?;
    transaction.commit().await?;
    Ok(ExecutionOutcome::TaskCompleted)
}
