use crate::error::ErrorCode;
use crate::state::player_profile::PlayerProfile;
use crate::utils::helpers::{is_native, transfer_sol, transfer_token};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PlayerProfileDepositArgs {
    pub amount: u64,
}

#[derive(Accounts)]
#[instruction(args: PlayerProfileDepositArgs)]
pub struct PlayerProfileDepositContext<'info> {
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
    seeds = [PlayerProfile::PREFIX.as_bytes(), signer.key().as_ref()],
    bump = player_profile.bump,
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
pub fn player_profile_deposit(
    ctx: Context<PlayerProfileDepositContext>,
    args: PlayerProfileDepositArgs,
) -> Result<()> {
    let signer = &mut ctx.accounts.signer;
    let signer_token_account = &ctx.accounts.signer_token_account;
    let player_profile = &mut ctx.accounts.player_profile;
    let player_token_account = &ctx.accounts.player_token_account;
    let mint = &ctx.accounts.mint;
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;

    require_keys_eq!(
        signer.key(),
        player_profile.player,
        ErrorCode::SignerIsNotPlayer
    );

    msg!("here 1 mint {:?}", mint.key());

    if is_native(&mint.to_account_info()) {
        msg!("here 2");
        transfer_sol(
            signer.to_account_info(),
            player_profile.to_account_info(),
            system_program.to_account_info(),
            args.amount,
        )?;
    } else {
        msg!("here 3");
        transfer_token(
            signer_token_account.to_account_info(),
            player_token_account.to_account_info(),
            token_program.to_account_info(),
            signer.to_account_info(),
            args.amount,
        )?;
    }

    Ok(())
}
