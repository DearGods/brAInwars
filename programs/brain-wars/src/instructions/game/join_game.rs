use crate::error::ErrorCode;
use crate::state::crank::Crank;
use crate::state::game::{Game, GameStatus};
use crate::state::player_profile::PlayerProfile;
use crate::state::players_actions::{Action, PlayerAction, PlayersActions};
use crate::utils::helpers::{is_native, transfer_sol_from_pda, transfer_token_pda};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct JoinGameGameArgs {
    pub timestamp: u64,
}

#[derive(Accounts)]
#[instruction(args: JoinGameGameArgs)]
pub struct JoinGameContext<'info> {
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
    token::mint = mint,
    token::authority = crank,
    seeds = [Crank::PREFIX.as_bytes(), mint.key().as_ref()],
    bump
    )]
    pub crank_token_account: Box<Account<'info, TokenAccount>>,
    #[account()]
    /// CHECK: can be any user, crank handles it
    pub player: UncheckedAccount<'info>,
    #[account(
    mut,
    has_one = player @ ErrorCode::PlayerProfileNotOfPlayer,
    seeds = [PlayerProfile::PREFIX.as_bytes(), player.key().as_ref()],
    bump = player_profile.bump
    )]
    pub player_profile: Box<Account<'info, PlayerProfile>>,
    #[account(
    mut,
    token::mint = mint,
    token::authority = player_profile,
    seeds = [PlayerProfile::PREFIX.as_bytes(), player.key().as_ref(), mint.key().as_ref()],
    bump
    )]
    pub player_token_account: Box<Account<'info, TokenAccount>>,
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
pub fn join_game(ctx: Context<JoinGameContext>, args: JoinGameGameArgs) -> Result<()> {
    let crank = &ctx.accounts.crank;
    let crank_token_account = &mut ctx.accounts.crank_token_account;
    let player = &ctx.accounts.player;
    let player_profile = &mut ctx.accounts.player_profile;
    let player_token_account = &ctx.accounts.player_token_account;
    let mint = &ctx.accounts.mint;
    let game = &mut ctx.accounts.game;
    let game_token_account = &mut ctx.accounts.game_token_account;
    let players_actions = &mut ctx.accounts.players_actions;
    let token_program = &ctx.accounts.token_program;
    game.latest_timestamp = args.timestamp;

    // require_eq!(
    //     game.game_status.clone(),
    //     GameStatus::WaitingForPlayers,
    //     ErrorCode::GameNotWaitingForPlayers
    // );
    // if game.waiting_for_players_start_time.is_some() {
    //     msg!(
    //         "waiting_for_players_start_time + wait_for_players_limit {} | time {}",
    //         game.waiting_for_players_start_time.unwrap() + game.wait_for_players_limit,
    //         args.timestamp
    //     );
    //     require_gt!(
    //         game.waiting_for_players_start_time.unwrap() + game.wait_for_players_limit,
    //         args.timestamp,
    //         ErrorCode::GameAlreadyStarted
    //     );
    // }
    players_actions.add_player_action(PlayerAction {
        player: player.key(),
        game_status: GameStatus::WaitingForPlayers,
        action: Action::Join(args.timestamp),
    });
    players_actions.add_player(player.key());
    players_actions.current_size =
        PlayersActions::recalculate_size(players_actions.current_size) as u64;

    if players_actions.players_actions.len() == 2 {
        game.waiting_for_players_start_time = Some(args.timestamp);
    }

    let fee = (game.entry_fee as f64 * crank.fee as f64 / 10_000.0f64) as u64;

    if is_native(&mint.to_account_info()) {
        transfer_sol_from_pda(
            &mut player_profile.to_account_info(),
            &mut game.to_account_info(),
            game.entry_fee,
        )?;

        transfer_sol_from_pda(
            &mut player_profile.to_account_info(),
            &mut crank.to_account_info(),
            fee,
        )?;
    } else {
        let player_key = player.key();
        let seeds = &[
            PlayerProfile::PREFIX.as_bytes(),
            player_key.as_ref(),
            &[player_profile.bump],
        ];
        transfer_token_pda(
            player_token_account.to_account_info(),
            game_token_account.to_account_info(),
            token_program.to_account_info(),
            player_profile.to_account_info(),
            game.entry_fee,
            &[seeds],
        )?;

        transfer_token_pda(
            player_token_account.to_account_info(),
            crank_token_account.to_account_info(),
            token_program.to_account_info(),
            player_profile.to_account_info(),
            fee,
            &[seeds],
        )?;
    }
    Ok(())
}
