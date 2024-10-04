use solana_sdk::pubkey::Pubkey;

pub fn get_crank_address() -> (Pubkey, u8) {
    let program_id = brain_wars::id();
    Pubkey::find_program_address(&[b"CRANK"], &program_id)
}

pub fn get_crank_token_address(mint: &Pubkey) -> (Pubkey, u8) {
    let program_id = brain_wars::id();
    Pubkey::find_program_address(&[b"CRANK", &mint.to_bytes()], &program_id)
}

pub fn get_game_address(id: u64) -> (Pubkey, u8) {
    let program_id = brain_wars::id();
    Pubkey::find_program_address(&[b"GAME", &id.to_le_bytes()], &program_id)
}

pub fn get_game_token_address(id: u64, mint: &Pubkey) -> (Pubkey, u8) {
    let program_id = brain_wars::id();
    Pubkey::find_program_address(&[b"GAME", &mint.to_bytes(), &id.to_le_bytes()], &program_id)
}

pub fn get_players_actions_address(id: u64) -> (Pubkey, u8) {
    let program_id = brain_wars::id();
    Pubkey::find_program_address(&[b"PLAYERS_ACTIONS", &id.to_le_bytes()], &program_id)
}

pub fn get_player_profile_address(player: &Pubkey) -> (Pubkey, u8) {
    let program_id = brain_wars::id();
    Pubkey::find_program_address(&[b"PLAYER_PROFILE", &player.to_bytes()], &program_id)
}

pub fn get_player_profile_token_address(player: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    let program_id = brain_wars::id();
    Pubkey::find_program_address(
        &[b"PLAYER_PROFILE", &player.to_bytes(), &mint.to_bytes()],
        &program_id,
    )
}
