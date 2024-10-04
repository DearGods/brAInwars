use crate::error::ErrorCode;
use crate::state::crank::Crank;
use crate::state::game::{Game, GameStatus};
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct StartGameStatusArgs {
    pub timestamp: u64,
}

#[derive(Accounts)]
#[instruction(args: StartGameStatusArgs)]
pub struct StartGameStatusContext<'info> {
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
    mut,
    seeds = [Game::PREFIX.as_bytes(), & game.game_id.to_le_bytes()],
    bump = game.bump
    )]
    pub game: Box<Account<'info, Game>>,
    #[account()]
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn start_game(ctx: Context<StartGameStatusContext>, args: StartGameStatusArgs) -> Result<()> {
    let game = &mut ctx.accounts.game;
    // require_eq!(
    //     game.game_status.clone(),
    //     GameStatus::WaitingForPlayers,
    //     ErrorCode::GameNotWaitingForPlayers
    // );
    game.start_time = Some(args.timestamp);
    game.latest_timestamp = args.timestamp;
    game.game_status = GameStatus::OnGoing;
    Ok(())
}
