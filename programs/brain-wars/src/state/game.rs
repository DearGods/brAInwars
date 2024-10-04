#![allow(clippy::derivable_impls)]

use anchor_lang::prelude::*;
use std::fmt::Display;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq, Debug)]
pub enum GameStatus {
    WaitingForPlayers,
    OnGoing,
    Finished,
    Settled,
}

impl Default for GameStatus {
    fn default() -> Self {
        GameStatus::WaitingForPlayers
    }
}

impl Display for GameStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameStatus::WaitingForPlayers => write!(f, "WaitingForPlayers"),
            GameStatus::OnGoing => write!(f, "OnGoing"),
            GameStatus::Finished => write!(f, "Finished"),
            GameStatus::Settled => write!(f, "Settled"),
        }
    }
}

#[account]
#[derive(Default, Debug)]
pub struct Game {
    pub bump: u8,
    pub game_id: u64,
    pub entry_fee: u64,
    pub mint: Pubkey,
    pub start_time: Option<u64>,
    pub waiting_for_players_start_time: Option<u64>,
    pub winner: Option<Pubkey>,
    pub game_status: GameStatus,
    pub wait_for_players_limit: u64,
    pub players_actions: Pubkey,
    pub hashed_limit: u64,
    pub reveled_limit: Option<u64>,
    pub reveled_salt: Option<u64>,
    pub latest_timestamp: u64,
}

impl Game {
    pub const PREFIX: &'static str = "GAME";

    pub const SIZE: usize = 8 + /* discriminator */
        std::mem::size_of::<u8>() + /* bump */
        std::mem::size_of::<u64>() + /* game_id */
        std::mem::size_of::<u64>() + /* entry_fee */
        std::mem::size_of::<Pubkey>() + /* mint */
        std::mem::size_of::<Option<u64>>() + /* start_time */
        std::mem::size_of::<Option<u64>>() + /* waiting_for_players_start_time */
        std::mem::size_of::<Option<Pubkey>>() + /* winner */
        std::mem::size_of::<GameStatus>() + /* game_status */
        std::mem::size_of::<u64>() + /* wait_for_players_limit */
        std::mem::size_of::<Pubkey>() + /* players_actions */
        std::mem::size_of::<u64>() + /* hashed_limit */
        std::mem::size_of::<Option<u64>>() + /* reveled_limit */
        std::mem::size_of::<Option<u64>>() + /* reveled_salt */
        std::mem::size_of::<u64>(); /* latest_timestamp */

    pub fn change_from_waiting_to_ongoing(&mut self, time: u64) {
        if self.game_status != GameStatus::WaitingForPlayers {
            return;
        }

        if self.waiting_for_players_start_time.is_none() {
            return;
        }

        if time > self.waiting_for_players_start_time.unwrap() + self.wait_for_players_limit {
            msg!(
                "Changing game status to OnGoing: time: {} | waiting_for_players_start_time: {} | wait_for_players_limit: {}",
                time,
                self.waiting_for_players_start_time.unwrap(),
                self.wait_for_players_limit
            );
            self.game_status = GameStatus::OnGoing;
            self.start_time = Some(time);
        }
    }

    pub fn change_from_on_going_to_finished(&mut self, time: u64) {
        if self.game_status != GameStatus::OnGoing {
            return;
        }

        if self.start_time.is_none() || self.reveled_limit.is_none() {
            return;
        }

        if time > self.start_time.unwrap() + self.reveled_limit.unwrap() {
            msg!(
                "Changing game status to Finished: time: {} | start_time: {} | reveled_limit: {}",
                time,
                self.start_time.unwrap(),
                self.reveled_limit.unwrap()
            );
            self.game_status = GameStatus::Finished;
        }
    }
}
