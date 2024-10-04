use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Game no winner found")]
    GameNoWinnerFound,
    #[msg("Custom error message")]
    CustomError,
    #[msg("Signer not crank")]
    SignerNotCrank,
    #[msg("Mint not game token")]
    MintNotGameToken,
    #[msg("Game not players actions")]
    GameNotPlayersActions,
    #[msg("Game already ongoing")]
    GameAlreadyOngoing,
    #[msg("Game already finished")]
    GameAlreadyFinished,
    #[msg("Game already settled")]
    GameAlreadySettled,
    #[msg("Numerical overflow")]
    NumericalOverflow,
    #[msg("Player is not in game")]
    PlayerIsNotInGame,
    #[msg("Game is not ongoing")]
    GameNotOngoing,
    #[msg("Game is not finished")]
    GameNotFinished,
    #[msg("Player profile not of player")]
    PlayerProfileNotOfPlayer,
    #[msg("Signer is not player")]
    SignerIsNotPlayer,
    #[msg("Winner not winner")]
    WinnerNotWinner,
    #[msg("Game not waiting for players")]
    GameNotWaitingForPlayers,
    #[msg("Game already started")]
    GameAlreadyStarted,
    #[msg("Game not OnGoing or WaitingForPlayers")]
    GameNotOnGoingOrWaitingForPlayers,
    #[msg("Game hashed limit not matched")]
    GameHashedLimitNotMatched,
    #[msg("Game Already Has Winner")]
    GameAlreadyHasWinner,
    #[msg("Game not started")]
    GameNotStarted,
    #[msg("Wrong status for start game")]
    WrongStatusForStartGame,
}
