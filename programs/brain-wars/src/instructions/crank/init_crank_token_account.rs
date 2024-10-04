use crate::error::ErrorCode;
use crate::state::crank::Crank;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct InitCrankTokenAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    seeds = [Crank::PREFIX.as_bytes()],
    has_one = signer @ ErrorCode::SignerNotCrank,
    bump = crank.bump
    )]
    pub crank: Box<Account<'info, Crank>>,
    #[account(
    init,
    payer = signer,
    token::mint = mint,
    token::authority = crank,
    seeds = [Crank::PREFIX.as_bytes(), mint.key().as_ref()],
    bump
    )]
    pub crank_token_account: Box<Account<'info, TokenAccount>>,
    #[account()]
    pub mint: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn init_crank_token_account(ctx: Context<InitCrankTokenAccount>) -> Result<()> {
    let time = Clock::get().unwrap().unix_timestamp as u64;
    let mint = &ctx.accounts.mint;
    msg!("time {}", time);
    msg!("Creating crank_token_account for mint {:?}", mint.key());
    Ok(())
}
