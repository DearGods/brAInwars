use crate::blockchain::solana::helpers::get_anchor_client;
use crate::blockchain::solana::pdas::{get_crank_address, get_crank_token_address};
use anchor_lang::prelude::Pubkey;
use anchor_lang::solana_program::{system_program, sysvar};
use anchor_spl::associated_token;
use anchor_spl::token::spl_token;
use brain_wars::instruction as brain_wars_instruction;
use brain_wars::{accounts as brain_wars_account, SetCrankArgs};
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::{Keypair, Signer};
use std::sync::Arc;

pub async fn init_crank_instruction(signer: &Keypair) -> anyhow::Result<Vec<Instruction>> {
    let client = get_anchor_client(None);
    let program_id = brain_wars::id();
    let program = client.program(program_id).unwrap();
    let (crank, _) = get_crank_address();

    Ok(program
        .request()
        .accounts(brain_wars_account::InitCrankContext {
            signer: signer.pubkey(),
            crank_signer: signer.pubkey(),
            crank,
            system_program: system_program::ID,
            rent: sysvar::rent::ID,
        })
        .args(brain_wars_instruction::InitCrank {})
        .instructions()?)
}

pub async fn init_crank_token_instruction(
    signer: &Keypair,
    mint: &Pubkey,
) -> anyhow::Result<Vec<Instruction>> {
    let client = get_anchor_client(None);
    let program_id = brain_wars::id();
    let program = client.program(program_id).unwrap();
    let (crank, _) = get_crank_address();
    let (crank_token_account, _) = get_crank_token_address(mint);

    Ok(program
        .request()
        .accounts(brain_wars_account::InitCrankTokenAccount {
            signer: signer.pubkey(),
            crank,
            crank_token_account,
            mint: *mint,
            system_program: system_program::ID,
            token_program: spl_token::ID,
            associated_token_program: associated_token::ID,
            rent: sysvar::rent::ID,
        })
        .args(brain_wars_instruction::InitCrankTokenAccount {})
        .instructions()?)
}

pub async fn set_crank_instruction(
    signer: Arc<Keypair>,
    crank_signer: &Pubkey,
    fee: u64,
) -> anyhow::Result<Vec<Instruction>> {
    let client = get_anchor_client(None);
    let program_id = brain_wars::id();
    let program = client.program(program_id).unwrap();
    let (crank, _) = get_crank_address();

    Ok(program
        .request()
        .accounts(brain_wars_account::SetCrankContext {
            signer: signer.pubkey(),
            crank,
            crank_signer: *crank_signer,
            system_program: system_program::ID,
            rent: sysvar::rent::ID,
        })
        .args(brain_wars_instruction::SetCrank {
            args: SetCrankArgs { fee },
        })
        .instructions()?)
}
