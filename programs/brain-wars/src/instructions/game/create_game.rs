use crate::error::ErrorCode;
use crate::state::crank::Crank;
use crate::state::game::{Game, GameStatus};
use crate::state::players_actions::PlayersActions;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateGameArgs {
    pub game_id: u64,
    pub wait_for_players_limit: u64,
    pub hashed_limit: u64,
    pub entry_fee: u64,
    pub timestamp: u64,
}

#[derive(Accounts)]
#[instruction(args: CreateGameArgs)]
pub struct CreateGameContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    seeds = [Crank::PREFIX.as_bytes()],
    has_one = signer @ ErrorCode::SignerNotCrank,
    bump = crank.bump
    )]
    pub crank: Box<Account<'info, Crank>>,
    #[account(
    init,
    payer = signer,
    seeds = [Game::PREFIX.as_bytes(), & args.game_id.to_le_bytes()],
    space = Game::SIZE,
    bump
    )]
    pub game: Box<Account<'info, Game>>,
    #[account(
    init,
    payer = signer,
    token::mint = mint,
    token::authority = game,
    seeds = [Game::PREFIX.as_bytes(), mint.key().as_ref(), & args.game_id.to_le_bytes()],
    bump
    )]
    pub game_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
    init,
    payer = signer,
    seeds = [PlayersActions::PREFIX.as_bytes(), & args.game_id.to_le_bytes()],
    space = PlayersActions::SIZE,
    bump
    )]
    pub players_actions: Box<Account<'info, PlayersActions>>,
    #[account()]
    pub mint: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn create_game(ctx: Context<CreateGameContext>, args: CreateGameArgs) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let players_actions = &mut ctx.accounts.players_actions;
    let mint = &ctx.accounts.mint;

    game.bump = ctx.bumps.game;
    game.game_id = args.game_id;
    game.wait_for_players_limit = args.wait_for_players_limit;
    game.hashed_limit = args.hashed_limit;
    game.game_status = GameStatus::WaitingForPlayers;
    game.players_actions = players_actions.key();
    game.reveled_limit = None;
    game.reveled_salt = None;
    game.mint = mint.key();
    game.entry_fee = args.entry_fee;
    game.start_time = None;
    game.latest_timestamp = args.timestamp;

    players_actions.bump = ctx.bumps.players_actions;
    players_actions.current_size =
        PlayersActions::recalculate_size(PlayersActions::SIZE as u64) as u64;
    players_actions.game = game.key();
    players_actions.players_actions = vec![];

    Ok(())
}
