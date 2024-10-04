use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct PlayerProfile {
    pub bump: u8,
    pub player: Pubkey,
    pub games_played: u64,
    pub games_won: u64,
}

impl PlayerProfile {
    pub const PREFIX: &'static str = "PLAYER_PROFILE";

    pub const SIZE: usize = 8 + /* discriminator */
        std::mem::size_of::<u8>() + /* bump */
        std::mem::size_of::<Pubkey>() + /* player */
        std::mem::size_of::<u64>() + /* games_played */
        std::mem::size_of::<u64>() + /* games_won */
        64; /* padding */

    pub fn init(&mut self, bump: u8, player: Pubkey) {
        if self.bump != 0 {
            return;
        }
        self.bump = bump;
        self.player = player;
        self.games_played = 0;
        self.games_won = 0;
    }
}
