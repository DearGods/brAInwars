#![allow(ambiguous_glob_reexports)]

pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

pub use instructions::*;

declare_id!("2yKYJeX7NDF6gSHANJqnxHuAuHuBx5Qf3s9oAmMpWzxh");

#[program]
pub mod brain_wars {
    use super::*;

    pub fn init_player_profile(ctx: Context<InitPlayerProfileContext>) -> Result<()> {
        init_player_profile::init_player_profile(ctx)
    }

    pub fn init_signer_token_account(ctx: Context<InitSignerTokenAccount>) -> Result<()> {
        init_signer_token_account::init_signer_token_account(ctx)
    }

    pub fn init_crank_token_account(ctx: Context<InitCrankTokenAccount>) -> Result<()> {
        init_crank_token_account::init_crank_token_account(ctx)
    }

    pub fn init_player_profile_token_account(
        ctx: Context<InitPlayerProfileTokenAccountContext>,
    ) -> Result<()> {
        init_player_profile_token_account::init_player_profile_token_account(ctx)
    }

    pub fn init_crank(ctx: Context<InitCrankContext>) -> Result<()> {
        init_crank::init_crank(ctx)
    }

    pub fn create_game(ctx: Context<CreateGameContext>, args: CreateGameArgs) -> Result<()> {
        create_game::create_game(ctx, args)
    }

    pub fn join_game(ctx: Context<JoinGameContext>, args: JoinGameGameArgs) -> Result<()> {
        join_game::join_game(ctx, args)
    }

    pub fn leave_game(ctx: Context<LeaveGameContext>, args: LeaveGameGameArgs) -> Result<()> {
        leave_game::leave_game(ctx, args)
    }

    pub fn finish_game(ctx: Context<FinishGameContext>, args: FinishGameArgs) -> Result<()> {
        finish_game::finish_game(ctx, args)
    }

    pub fn settle_game(ctx: Context<SettleGameContext>, args: SettleGameGameArgs) -> Result<()> {
        settle_game::settle_game(ctx, args)
    }

    pub fn player_profile_deposit(
        ctx: Context<PlayerProfileDepositContext>,
        args: PlayerProfileDepositArgs,
    ) -> Result<()> {
        player_profile_deposit::player_profile_deposit(ctx, args)
    }

    pub fn player_profile_withdraw(
        ctx: Context<PlayerProfileWithdrawContext>,
        args: PlayerProfileWithdrawArgs,
    ) -> Result<()> {
        player_profile_withdraw::player_profile_withdraw(ctx, args)
    }

    pub fn set_crank(ctx: Context<SetCrankContext>, args: SetCrankArgs) -> Result<()> {
        set_crank::set_crank(ctx, args)
    }

    pub fn start_game(
        ctx: Context<StartGameStatusContext>,
        args: StartGameStatusArgs,
    ) -> Result<()> {
        start_game::start_game(ctx, args)
    }
}
