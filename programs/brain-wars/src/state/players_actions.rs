use crate::state::game::GameStatus;
use crate::utils::helpers::{set_to_vec, vec_to_set};
use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub enum Action {
    Join(u64),
    Leave(u64),
}

impl Action {
    pub fn get_time(&self) -> u64 {
        match self {
            Action::Join(time) => *time,
            Action::Leave(time) => *time,
        }
    }

    pub fn is_leave(&self) -> bool {
        match self {
            Action::Join(_) => false,
            Action::Leave(_) => true,
        }
    }

    pub fn is_join(&self) -> bool {
        match self {
            Action::Join(_) => true,
            Action::Leave(_) => false,
        }
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct PlayerAction {
    pub player: Pubkey,
    pub game_status: GameStatus,
    pub action: Action,
}

#[account]
#[derive(Default, Debug)]
pub struct PlayersActions {
    pub bump: u8,
    pub current_size: u64,
    pub game: Pubkey,
    pub players_actions: Vec<PlayerAction>,
    pub players: Vec<Pubkey>,
}

impl PlayersActions {
    pub const PREFIX: &'static str = "PLAYERS_ACTIONS";

    pub const SIZE: usize = 8 + /* discriminator */
        std::mem::size_of::<u8>() + /* bump */
        std::mem::size_of::<u64>() + /* current_size */
        std::mem::size_of::<Pubkey>() + /* game */
        std::mem::size_of::<Vec<PlayerAction>>() + /* player_actions */
        std::mem::size_of::<Vec<Pubkey>>() + /* players */
        64; /* padding */

    pub fn recalculate_size(current_size: u64) -> usize {
        (current_size
            + std::mem::size_of::<Vec<Pubkey>>() as u64
            + std::mem::size_of::<PlayerAction>() as u64
            + 64u64) as usize
    }

    pub fn is_player_in_game(&self, player: Pubkey) -> bool {
        self.players.contains(&player)
    }
    pub fn add_player(&mut self, player: Pubkey) {
        let mut h = vec_to_set(&self.players);
        h.insert(player.key());
        self.players = set_to_vec(&h);
    }

    pub fn remove_player(&mut self, player: Pubkey) {
        let mut h = vec_to_set(&self.players);
        h.remove(&player.key());
        self.players = set_to_vec(&h);
    }

    pub fn add_player_action(&mut self, player_action: PlayerAction) {
        self.players_actions.push(player_action);
    }
}
