use crate::error::ErrorCode;
use crate::state::crank::Crank;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetCrankArgs {
    pub fee: u64,
}

#[derive(Accounts)]
#[instruction(args: SetCrankArgs)]
pub struct SetCrankContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account()]
    /// CHECK: can be any target
    pub crank_signer: UncheckedAccount<'info>,
    #[account(
    mut,
    seeds = [Crank::PREFIX.as_bytes()],
    bump = crank.bump
    )]
    pub crank: Box<Account<'info, Crank>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn set_crank(ctx: Context<SetCrankContext>, args: SetCrankArgs) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let crank = &mut ctx.accounts.crank;
    let crank_signer = &ctx.accounts.crank_signer;

    if crank.signer != Pubkey::default() {
        require_keys_eq!(signer.key(), crank.signer, ErrorCode::SignerNotCrank);
    }

    crank.signer = crank_signer.key();
    crank.fee = args.fee;
    Ok(())
}
