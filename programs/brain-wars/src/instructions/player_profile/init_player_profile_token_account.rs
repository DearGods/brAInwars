use crate::state::player_profile::PlayerProfile;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct InitPlayerProfileTokenAccountContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    seeds = [PlayerProfile::PREFIX.as_bytes(), signer.key().as_ref()],
    bump = player_profile.bump
    )]
    pub player_profile: Box<Account<'info, PlayerProfile>>,
    #[account(
    init,
    payer = signer,
    token::mint = mint,
    token::authority = player_profile,
    seeds = [PlayerProfile::PREFIX.as_bytes(), signer.key().as_ref(), mint.key().as_ref()],
    bump
    )]
    pub player_token_account: Box<Account<'info, TokenAccount>>,
    #[account()]
    pub mint: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn init_player_profile_token_account(
    _ctx: Context<InitPlayerProfileTokenAccountContext>,
) -> Result<()> {
    Ok(())
}
