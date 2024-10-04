use crate::blockchain::solana::helpers::get_anchor_client;
use crate::blockchain::solana::pdas::{
    get_crank_address, get_crank_token_address, get_game_address, get_game_token_address,
    get_player_profile_address, get_player_profile_token_address, get_players_actions_address,
};
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::sysvar;
use anchor_lang::system_program;
use anchor_spl::associated_token;
use anchor_spl::token::spl_token;
use brain_wars::{accounts as brain_wars_account, CreateGameArgs, FinishGameArgs};
use brain_wars::{
    instruction as brain_wars_instruction, JoinGameGameArgs, LeaveGameGameArgs, SettleGameGameArgs,
    StartGameStatusArgs,
};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

// https://github.com/coral-xyz/anchor/blob/bcd7e1719e5b4958792d2165bfaf7020b75530b0/client/example/src/blocking.rs
#[tracing::instrument(name = "create_game_instruction", skip(signer), ret, err)]
pub fn create_game_instruction(
    signer: &Keypair,
    game_id: u64,
    wait_for_players_limit: u64,
    hashed_limit: u64,
    entry_fee: u64,
    mint: &Pubkey,
    timestamp: u64,
) -> anyhow::Result<Vec<Instruction>> {
    let client = get_anchor_client(None);
    let program_id = brain_wars::id();
    let program = client.program(program_id).unwrap();
    let (crank, _) = get_crank_address();
    let (game, _) = get_game_address(game_id);
    let (game_token_account, _) = get_game_token_address(game_id, mint);
    let (players_actions, _) = get_players_actions_address(game_id);
    Ok(program
        .request()
        .accounts(brain_wars_account::CreateGameContext {
            signer: signer.pubkey(),
            crank,
            game,
            game_token_account,
            players_actions,
            mint: *mint,
            system_program: system_program::ID,
            token_program: spl_token::ID,
            associated_token_program: associated_token::ID,
            rent: sysvar::rent::ID,
        })
        .args(brain_wars_instruction::CreateGame {
            args: CreateGameArgs {
                timestamp,
                game_id,
                wait_for_players_limit,
                hashed_limit,
                entry_fee,
            },
        })
        .instructions()?)
}

#[tracing::instrument(name = "join_game_instruction", skip(signer), ret, err)]
pub fn join_game_instruction(
    signer: &Keypair,
    player: &Pubkey,
    game_id: u64,
    mint: &Pubkey,
    timestamp: u64,
) -> anyhow::Result<Vec<Instruction>> {
    let client = get_anchor_client(None);
    let program_id = brain_wars::id();
    let program = client.program(program_id).unwrap();
    let (crank, _) = get_crank_address();
    let (crank_token_account, _) = get_crank_token_address(mint);
    let (game, _) = get_game_address(game_id);
    let (game_token_account, _) = get_game_token_address(game_id, mint);
    let (players_actions, _) = get_players_actions_address(game_id);
    let (player_profile, _) = get_player_profile_address(player);
    let (player_token_account, _) = get_player_profile_token_address(player, mint);

    Ok(program
        .request()
        .accounts(brain_wars_account::JoinGameContext {
            signer: signer.pubkey(),
            crank,
            game,
            game_token_account,
            crank_token_account,
            player: *player,
            player_profile,
            player_token_account,
            players_actions,
            mint: *mint,
            system_program: system_program::ID,
            token_program: spl_token::ID,
            associated_token_program: associated_token::ID,
            rent: sysvar::rent::ID,
        })
        .args(brain_wars_instruction::JoinGame {
            args: JoinGameGameArgs { timestamp },
        })
        .instructions()?)
}

#[tracing::instrument(name = "leave_game_instruction", skip(signer), ret, err)]
pub fn leave_game_instruction(
    signer: &Keypair,
    player: &Pubkey,
    game_id: u64,
    mint: &Pubkey,
    timestamp: u64,
) -> anyhow::Result<Vec<Instruction>> {
    let client = get_anchor_client(None);
    let program_id = brain_wars::id();
    let program = client.program(program_id).unwrap();
    let (crank, _) = get_crank_address();
    let (game, _) = get_game_address(game_id);
    let (game_token_account, _) = get_game_token_address(game_id, mint);
    let (players_actions, _) = get_players_actions_address(game_id);
    let (player_profile, _) = get_player_profile_address(player);
    let (player_token_account, _) = get_player_profile_token_address(player, mint);

    Ok(program
        .request()
        .accounts(brain_wars_account::LeaveGameContext {
            signer: signer.pubkey(),
            crank,
            game,
            game_token_account,
            player: *player,
            player_profile,
            player_token_account,
            players_actions,
            mint: *mint,
            system_program: system_program::ID,
            token_program: spl_token::ID,
            associated_token_program: associated_token::ID,
            rent: sysvar::rent::ID,
        })
        .args(brain_wars_instruction::LeaveGame {
            args: LeaveGameGameArgs { timestamp },
        })
        .instructions()?)
}

#[tracing::instrument(name = "settle_game_instruction", skip(signer), ret, err)]
pub fn settle_game_instruction(
    signer: &Keypair,
    winner: &Pubkey,
    game_id: u64,
    mint: &Pubkey,
    timestamp: u64,
) -> anyhow::Result<Vec<Instruction>> {
    let client = get_anchor_client(None);
    let program_id = brain_wars::id();
    let program = client.program(program_id).unwrap();
    let (crank, _) = get_crank_address();
    let (game, _) = get_game_address(game_id);
    let (game_token_account, _) = get_game_token_address(game_id, mint);
    let (players_actions, _) = get_players_actions_address(game_id);
    let (player_profile, _) = get_player_profile_address(winner);
    let (player_token_account, _) = get_player_profile_token_address(winner, mint);

    Ok(program
        .request()
        .accounts(brain_wars_account::SettleGameContext {
            signer: signer.pubkey(),
            crank,
            game,
            winner: *winner,
            game_token_account,
            player_profile,
            player_token_account,
            players_actions,
            mint: *mint,
            system_program: system_program::ID,
            token_program: spl_token::ID,
            associated_token_program: associated_token::ID,
            rent: sysvar::rent::ID,
        })
        .args(brain_wars_instruction::SettleGame {
            args: SettleGameGameArgs { timestamp },
        })
        .instructions()?)
}

#[tracing::instrument(name = "finish_game_instruction", skip(signer), ret, err)]
pub fn finish_game_instruction(
    signer: &Keypair,
    game_id: u64,
    reveled_limit: u64,
    reveled_salt: u64,
    timestamp: u64,
    winner: &Pubkey,
) -> anyhow::Result<Vec<Instruction>> {
    let client = get_anchor_client(None);
    let program_id = brain_wars::id();
    let program = client.program(program_id).unwrap();
    let (crank, _) = get_crank_address();
    let (game, _) = get_game_address(game_id);
    let (players_actions, _) = get_players_actions_address(game_id);

    Ok(program
        .request()
        .accounts(brain_wars_account::FinishGameContext {
            signer: signer.pubkey(),
            winner: *winner,
            crank,
            game,
            players_actions,
            system_program: system_program::ID,
            rent: sysvar::rent::ID,
        })
        .args(brain_wars_instruction::FinishGame {
            args: FinishGameArgs {
                reveled_limit,
                reveled_salt,
                timestamp,
            },
        })
        .instructions()?)
}

#[tracing::instrument(name = "start_game_instruction", skip(signer), ret, err)]
pub fn start_game_instruction(
    signer: &Keypair,
    game_id: u64,
    timestamp: u64,
) -> anyhow::Result<Vec<Instruction>> {
    let client = get_anchor_client(None);
    let program_id = brain_wars::id();
    let program = client.program(program_id).unwrap();
    let (crank, _) = get_crank_address();
    let (game, _) = get_game_address(game_id);

    Ok(program
        .request()
        .accounts(brain_wars_account::StartGameStatusContext {
            signer: signer.pubkey(),
            crank,
            game,
            system_program: system_program::ID,
            rent: sysvar::rent::ID,
        })
        .args(brain_wars_instruction::StartGame {
            args: StartGameStatusArgs { timestamp },
        })
        .instructions()?)
}
