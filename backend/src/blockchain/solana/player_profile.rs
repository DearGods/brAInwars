use crate::blockchain::solana::helpers::get_anchor_client;
use crate::blockchain::solana::pdas::{
    get_player_profile_address, get_player_profile_token_address,
};
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::sysvar;
use anchor_lang::system_program;
use anchor_spl::associated_token;
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::token::spl_token;
use brain_wars::instruction as brain_wars_instruction;
use brain_wars::{
    accounts as brain_wars_account, PlayerProfileDepositArgs, PlayerProfileWithdrawArgs,
};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

pub fn init_signer_token_account(
    signer: &Keypair,
    mint: &Pubkey,
) -> anyhow::Result<Vec<Instruction>> {
    let client = get_anchor_client(None);
    let program_id = brain_wars::id();
    let program = client.program(program_id).unwrap();
    let signer_token_account = get_associated_token_address(&signer.pubkey(), mint);

    Ok(program
        .request()
        .accounts(brain_wars_account::InitSignerTokenAccount {
            signer: signer.pubkey(),
            signer_token_account,
            mint: *mint,
            system_program: system_program::ID,
            token_program: spl_token::ID,
            associated_token_program: associated_token::ID,
            rent: sysvar::rent::ID,
        })
        .args(brain_wars_instruction::InitSignerTokenAccount {})
        .instructions()?)
}

pub fn init_player_profile(signer: &Keypair, mint: &Pubkey) -> anyhow::Result<Vec<Instruction>> {
    let client = get_anchor_client(None);
    let program_id = brain_wars::id();
    let program = client.program(program_id).unwrap();
    let (player_profile, _) = get_player_profile_address(&signer.pubkey());
    Ok(program
        .request()
        .accounts(brain_wars_account::InitPlayerProfileContext {
            signer: signer.pubkey(),
            player_profile,
            mint: *mint,
            system_program: system_program::ID,
            token_program: spl_token::ID,
            associated_token_program: associated_token::ID,
            rent: sysvar::rent::ID,
        })
        .args(brain_wars_instruction::InitPlayerProfile {})
        .instructions()?)
}

pub fn init_player_profile_token_account_instruction(
    signer: &Keypair,
    mint: &Pubkey,
) -> anyhow::Result<Vec<Instruction>> {
    let client = get_anchor_client(None);
    let program_id = brain_wars::id();
    let program = client.program(program_id).unwrap();
    let (player_profile, _) = get_player_profile_address(&signer.pubkey());
    let (player_token_account, _) = get_player_profile_token_address(&signer.pubkey(), mint);

    Ok(program
        .request()
        .accounts(brain_wars_account::InitPlayerProfileTokenAccountContext {
            signer: signer.pubkey(),
            player_profile,
            player_token_account,
            mint: *mint,
            system_program: system_program::ID,
            token_program: spl_token::ID,
            associated_token_program: associated_token::ID,
            rent: sysvar::rent::ID,
        })
        .args(brain_wars_instruction::InitPlayerProfileTokenAccount {})
        .instructions()?)
}

pub fn player_profile_deposit_instruction(
    signer: &Keypair,
    mint: &Pubkey,
    amount: u64,
) -> anyhow::Result<Vec<Instruction>> {
    let client = get_anchor_client(None);
    let program_id = brain_wars::id();
    let program = client.program(program_id).unwrap();
    let signer_token_account = get_associated_token_address(&signer.pubkey(), mint);
    let (player_profile, _) = get_player_profile_address(&signer.pubkey());
    let (player_token_account, _) = get_player_profile_token_address(&signer.pubkey(), mint);
    Ok(program
        .request()
        .accounts(brain_wars_account::PlayerProfileDepositContext {
            signer: signer.pubkey(),
            signer_token_account,
            player_profile,
            player_token_account,
            mint: *mint,
            system_program: system_program::ID,
            token_program: spl_token::ID,
            associated_token_program: associated_token::ID,
            rent: sysvar::rent::ID,
        })
        .args(brain_wars_instruction::PlayerProfileDeposit {
            args: PlayerProfileDepositArgs { amount },
        })
        .instructions()?)
}

pub fn player_profile_withdraw_instruction(
    signer: &Keypair,
    mint: &Pubkey,
    amount: u64,
) -> anyhow::Result<Vec<Instruction>> {
    let client = get_anchor_client(None);
    let program_id = brain_wars::id();
    let program = client.program(program_id).unwrap();
    let signer_token_account = get_associated_token_address(&signer.pubkey(), mint);
    let (player_profile, _) = get_player_profile_address(&signer.pubkey());
    let (player_token_account, _) = get_player_profile_token_address(&signer.pubkey(), mint);
    Ok(program
        .request()
        .accounts(brain_wars_account::PlayerProfileWithdrawContext {
            signer: signer.pubkey(),
            signer_token_account,
            player_profile,
            player_token_account,
            mint: *mint,
            system_program: system_program::ID,
            token_program: spl_token::ID,
            associated_token_program: associated_token::ID,
            rent: sysvar::rent::ID,
        })
        .args(brain_wars_instruction::PlayerProfileWithdraw {
            args: PlayerProfileWithdrawArgs { amount },
        })
        .instructions()?)
}
