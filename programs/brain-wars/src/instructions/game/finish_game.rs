use crate::error::ErrorCode;
use crate::state::crank::Crank;
use crate::state::game::{Game, GameStatus};
use crate::state::players_actions::PlayersActions;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::hash;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct FinishGameArgs {
    pub reveled_limit: u64,
    pub reveled_salt: u64,
    pub timestamp: u64,
}

#[derive(Accounts)]
#[instruction(args: FinishGameArgs)]
pub struct FinishGameContext<'info> {
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
    seeds = [Game::PREFIX.as_bytes(), & game.game_id.to_le_bytes()],
    bump = game.bump
    )]
    pub game: Box<Account<'info, Game>>,
    #[account()]
    /// CHECK: Input from backend
    pub winner: AccountInfo<'info>,
    #[account(
    has_one = game @ ErrorCode::GameNotPlayersActions,
    seeds = [PlayersActions::PREFIX.as_bytes(), & game.game_id.to_le_bytes()],
    bump = players_actions.bump
    )]
    pub players_actions: Box<Account<'info, PlayersActions>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn finish_game(ctx: Context<FinishGameContext>, args: FinishGameArgs) -> Result<()> {
    let game = &mut ctx.accounts.game;
    // let players_actions = &mut ctx.accounts.players_actions;
    game.latest_timestamp = args.timestamp;

    let _statuses = vec![
        GameStatus::OnGoing,
        GameStatus::WaitingForPlayers,
        GameStatus::Finished,
    ];
    // require_eq!(
    //     statuses.contains(&game.game_status),
    //     true,
    //     ErrorCode::GameNotOnGoingOrWaitingForPlayers
    // );
    if game.winner.is_some() && game.winner.unwrap() != ctx.accounts.winner.key() {
        require_eq!(game.winner.is_none(), true, ErrorCode::GameAlreadyHasWinner);
    }

    require_eq!(game.start_time.is_some(), true, ErrorCode::GameNotStarted);

    game.game_status = GameStatus::Finished;
    game.reveled_limit = Option::from(args.reveled_limit);
    game.reveled_salt = Option::from(args.reveled_salt);

    let hash_input = format!("{}{}{}", game.key(), args.reveled_limit, args.reveled_salt);

    let h = hash(hash_input.as_bytes());
    let mut sum = 0u64;
    h.0.iter().for_each(|byte| {
        sum += *byte as u64;
    });

    require_eq!(sum, game.hashed_limit, ErrorCode::GameHashedLimitNotMatched);

    // let mut winner: Option<PlayerAction> = None;
    // for player_action in players_actions.players_actions.iter() {
    //     if player_action.action.is_join() {
    //         continue;
    //     }
    //     if player_action.game_status != GameStatus::OnGoing {
    //         msg!("Player action status isn't ongoing");
    //         continue;
    //     }
    //     if !players_actions.is_player_in_game(player_action.player) {
    //         msg!("Player not in game");
    //         continue;
    //     }
    //     if game.start_time.unwrap() + game.reveled_limit.unwrap() < player_action.action.get_time()
    //     {
    //         msg!(
    //             "Player reveled after limit {} | {} | {}",
    //             game.start_time.unwrap(),
    //             game.reveled_limit.unwrap(),
    //             player_action.action.get_time()
    //         );
    //         continue;
    //     }
    //     winner = Some(player_action.clone());
    // }
    // game.winner = winner.map(|player_action| player_action.player);
    game.winner = Some(ctx.accounts.winner.key());
    game.game_status = GameStatus::Finished;
    Ok(())
}
