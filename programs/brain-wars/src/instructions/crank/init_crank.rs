use crate::state::crank::Crank;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitCrankContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account()]
    /// CHECK: can be any target
    pub crank_signer: UncheckedAccount<'info>,
    #[account(
    init,
    payer = signer,
    space = Crank::SIZE,
    seeds = [Crank::PREFIX.as_bytes()],
    bump
    )]
    pub crank: Box<Account<'info, Crank>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn init_crank(ctx: Context<InitCrankContext>) -> Result<()> {
    let crank = &mut ctx.accounts.crank;
    let crank_signer = &ctx.accounts.crank_signer;
    crank.bump = ctx.bumps.crank;
    crank.signer = crank_signer.key();
    crank.fee = 0;
    Ok(())
}
