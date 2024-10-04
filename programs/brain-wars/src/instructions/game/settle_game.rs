use crate::error::ErrorCode;
use crate::state::crank::Crank;
use crate::state::game::{Game, GameStatus};
use crate::state::player_profile::PlayerProfile;
use crate::state::players_actions::PlayersActions;
use crate::utils::helpers::{is_native, transfer_sol_from_pda, transfer_token_pda};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SettleGameGameArgs {
    pub timestamp: u64,
}

#[derive(Accounts)]
#[instruction(args: SettleGameGameArgs)]
pub struct SettleGameContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    seeds = [Crank::PREFIX.as_bytes()],
    has_one = signer @ ErrorCode::SignerNotCrank,
    bump = crank.bump
    )]
    pub crank: Box<Account<'info, Crank>>,
    #[account(
    mut,
    address = game.winner.unwrap())]
    /// CHECK: Checking address
    pub winner: AccountInfo<'info>,
    #[account(
    mut,
    seeds = [Game::PREFIX.as_bytes(), & game.game_id.to_le_bytes()],
    has_one = mint @ ErrorCode::MintNotGameToken,
    bump = game.bump
    )]
    pub game: Box<Account<'info, Game>>,
    #[account(
    mut,
    token::mint = mint,
    token::authority = game,
    seeds = [Game::PREFIX.as_bytes(), mint.key().as_ref(), & game.game_id.to_le_bytes()],
    bump
    )]
    pub game_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
    mut,
    constraint = winner.key() == player_profile.player @ ErrorCode::PlayerProfileNotOfPlayer,
    seeds = [PlayerProfile::PREFIX.as_bytes(), winner.key().as_ref()],
    bump = player_profile.bump
    )]
    pub player_profile: Box<Account<'info, PlayerProfile>>,
    #[account(
    mut,
    token::mint = mint,
    token::authority = player_profile,
    seeds = [PlayerProfile::PREFIX.as_bytes(), winner.key().as_ref(), mint.key().as_ref()],
    bump
    )]
    pub player_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
    mut,
    has_one = game @ ErrorCode::GameNotPlayersActions,
    seeds = [PlayersActions::PREFIX.as_bytes(), & game.game_id.to_le_bytes()],
    realloc = PlayersActions::recalculate_size(players_actions.current_size),
    realloc::payer = signer,
    realloc::zero = false,
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
pub fn settle_game(ctx: Context<SettleGameContext>, args: SettleGameGameArgs) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let winner = &ctx.accounts.winner;
    let players_actions = &mut ctx.accounts.players_actions;
    let player_profile = &mut ctx.accounts.player_profile;
    let player_token_account = &mut ctx.accounts.player_token_account;
    let mint = &ctx.accounts.mint;
    let game = &mut ctx.accounts.game;
    let game_token_account = &mut ctx.accounts.game_token_account;
    let token_program = &ctx.accounts.token_program;
    game.latest_timestamp = args.timestamp;
    require_keys_eq!(
        winner.key(),
        game.winner.unwrap(),
        ErrorCode::WinnerNotWinner
    );
    // require_eq!(
    //     game.game_status.clone(),
    //     GameStatus::Finished,
    //     ErrorCode::GameNotFinished
    // );
    if game.winner.unwrap() != signer.key() {
        if is_native(&mint.to_account_info()) {
            transfer_sol_from_pda(
                &mut game.to_account_info(),
                &mut player_profile.to_account_info(),
                game.entry_fee * players_actions.players.len() as u64,
            )?;
        } else {
            let seeds = &[
                Game::PREFIX.as_bytes(),
                &game.game_id.to_le_bytes(),
                &[game.bump],
            ];

            transfer_token_pda(
                game_token_account.to_account_info(),
                player_token_account.to_account_info(),
                token_program.to_account_info(),
                game.to_account_info(),
                game.entry_fee * players_actions.players.len() as u64,
                &[seeds],
            )?;
        }
    }
    game.game_status = GameStatus::Settled;
    Ok(())
}
