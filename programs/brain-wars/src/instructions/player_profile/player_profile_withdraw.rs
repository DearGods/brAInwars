use crate::error::ErrorCode;
use crate::state::player_profile::PlayerProfile;
use crate::utils::helpers::{is_native, transfer_sol_from_pda, transfer_token_pda};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PlayerProfileWithdrawArgs {
    pub amount: u64,
}

#[derive(Accounts)]
#[instruction(args: PlayerProfileWithdrawArgs)]
pub struct PlayerProfileWithdrawContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = signer,
    )]
    pub signer_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
    mut,
    constraint = signer.key() == player_profile.player @ ErrorCode::SignerIsNotPlayer,
    seeds = [PlayerProfile::PREFIX.as_bytes(), signer.key().as_ref()],
    bump = player_profile.bump
    )]
    pub player_profile: Box<Account<'info, PlayerProfile>>,
    #[account(
    mut,
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
pub fn player_profile_withdraw(
    ctx: Context<PlayerProfileWithdrawContext>,
    args: PlayerProfileWithdrawArgs,
) -> Result<()> {
    let signer = &mut ctx.accounts.signer;
    let signer_token_account = &ctx.accounts.signer_token_account;
    let player_profile = &mut ctx.accounts.player_profile;
    let player_token_account = &ctx.accounts.player_token_account;
    let mint = &ctx.accounts.mint;
    let token_program = &ctx.accounts.token_program;

    if is_native(&mint.to_account_info()) {
        transfer_sol_from_pda(
            &mut player_profile.to_account_info(),
            &mut signer.to_account_info(),
            args.amount,
        )?;
    } else {
        let signer_key = signer.key();
        let seeds = &[
            PlayerProfile::PREFIX.as_bytes(),
            signer_key.as_ref(),
            &[player_profile.bump],
        ];
        transfer_token_pda(
            player_token_account.to_account_info(),
            signer_token_account.to_account_info(),
            token_program.to_account_info(),
            player_profile.to_account_info(),
            args.amount,
            &[seeds],
        )?;
    }

    Ok(())
}
