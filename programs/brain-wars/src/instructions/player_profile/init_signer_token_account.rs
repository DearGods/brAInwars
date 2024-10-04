use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct InitSignerTokenAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init,
    payer = signer,
    associated_token::mint = mint,
    associated_token::authority = signer,
    )]
    pub signer_token_account: Box<Account<'info, TokenAccount>>,
    #[account()]
    pub mint: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn init_signer_token_account(_ctx: Context<InitSignerTokenAccount>) -> Result<()> {
    Ok(())
}
