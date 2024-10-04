use crate::state::player_profile::PlayerProfile;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct InitPlayerProfileContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init,
    payer = signer,
    space = PlayerProfile::SIZE,
    seeds = [PlayerProfile::PREFIX.as_bytes(), signer.key().as_ref()],
    bump
    )]
    pub player_profile: Box<Account<'info, PlayerProfile>>,
    #[account()]
    pub mint: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn init_player_profile(ctx: Context<InitPlayerProfileContext>) -> Result<()> {
    let signer = &mut ctx.accounts.signer;
    let player_profile = &mut ctx.accounts.player_profile;
    player_profile.bump = ctx.bumps.player_profile;
    player_profile.player = signer.key();
    player_profile.games_played = 0;
    player_profile.games_won = 0;
    Ok(())
}
