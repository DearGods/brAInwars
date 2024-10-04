use axum::http::StatusCode;
use axum::response::IntoResponse;
use solana_sdk::pubkey::ParsePubkeyError;
use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("U64 must be positive")]
    U64MustBePositive,
    #[error("Bad float: {0}")]
    BadFloat(String),
    #[error("Gas too low")]
    GasTooLow,
    #[error("Failed to cast config to KeyValue due to: {0}")]
    FailedToCastConfigToKeyValue(String),
    #[error("ConfigNotFound: {0}")]
    ConfigNotFound(String),
    #[error("Bad environment variable")]
    BadEnvironmentVariable,
    #[error("Error verifying signature due to: {0}")]
    SignatureVerificationError(String),
    #[error("Error verifying transaction due to: {0}")]
    TransactionVerificationError(String),
    #[error("Get balance error")]
    GetBalanceError,
    #[error(transparent)]
    RandomGeneratorError(#[from] ParseIntError),
    #[error("Game not in progress")]
    GameNotInProgress,
    #[error("User mismatch")]
    UserMismatch,
    #[error("Game ended")]
    GameEnded,
    #[error("Game status unknown")]
    GameStatusUnknown,
    #[error("Game already started")]
    GameAlreadyStarted,
    #[error("Game MEV locked")]
    GameMevLocked,
    #[error("You're not the only player in the game")]
    NotOnlyPlayerInGame,
    #[error("Game not started yet")]
    GameNotStartedYet,
    #[error("Game still waiting for others to start")]
    GameStillWaitingForOthersToStart,
    #[error("Not enough balance")]
    NotEnoughBalance,
    #[error("Invalid balance change due to: {0}")]
    InvalidBalanceChange(String),
    #[error("Invalid currency")]
    InvalidCurrency,
    #[error(transparent)]
    ThreadError(#[from] tokio::task::JoinError),
    #[error("Password compute error")]
    PasswordComputeError,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Game longer than allowed")]
    GameLongerThanAllowed,
    #[error("Game shorter than allowed")]
    GameShorterThanAllowed,
    #[error("Game length must be positive")]
    GameLengthMustBePositive,
    #[error("Entry fee must be positive")]
    EntryFeeMustBePositive,
    #[error("User exists")]
    UserExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Nonce not found")]
    NonceNotFound,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error(transparent)]
    SqlError(#[from] sqlx::Error),
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error("{0} is not a valid user email.")]
    InvalidEmail(String),
    #[error("{0} is not a valid User name.")]
    InvalidUserName(String),
    #[error(transparent)]
    ParsePubkeyError(#[from] ParsePubkeyError),
    #[error("Authentication error: {0}")]
    AuthError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Error occurred: {}", self);
        match self {
            Error::NonceNotFound => (StatusCode::NOT_FOUND, "Nonce not found.").into_response(),
            Error::AuthError(source) => (StatusCode::UNAUTHORIZED, source).into_response(),
            Error::GameMevLocked => (StatusCode::BAD_REQUEST, "Game MEV locked.").into_response(),
            Error::U64MustBePositive => {
                (StatusCode::BAD_REQUEST, "U64 must be positive.").into_response()
            }
            Error::GameStatusUnknown => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Game status unknown.").into_response()
            }
            Error::GameAlreadyStarted => {
                (StatusCode::BAD_REQUEST, "Game already started.").into_response()
            }
            Error::ParsePubkeyError(_) => {
                (StatusCode::BAD_REQUEST, "Failed to parse pubkey").into_response()
            }
            Error::BadFloat(source) => (StatusCode::BAD_REQUEST, source).into_response(),
            Error::GasTooLow => (StatusCode::BAD_REQUEST, "Gas too low.").into_response(),
            Error::GetBalanceError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Get balance error.").into_response()
            }
            Error::FailedToCastConfigToKeyValue(source) => {
                (StatusCode::INTERNAL_SERVER_ERROR, source).into_response()
            }
            Error::ConfigNotFound(source) => {
                (StatusCode::INTERNAL_SERVER_ERROR, source).into_response()
            }
            Error::NotOnlyPlayerInGame => (
                StatusCode::BAD_REQUEST,
                "You're not the only player in the game.",
            )
                .into_response(),
            Error::BadEnvironmentVariable => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Bad environment variable.",
            )
                .into_response(),
            Error::SignatureVerificationError(_) => {
                (StatusCode::BAD_REQUEST, "Error verifying signature.").into_response()
            }
            Error::TransactionVerificationError(_) => {
                (StatusCode::BAD_REQUEST, "Error verifying transaction.").into_response()
            }
            Error::GameStillWaitingForOthersToStart => (
                StatusCode::BAD_REQUEST,
                "Game still waiting for others to start.",
            )
                .into_response(),
            Error::RandomGeneratorError(source) => {
                (StatusCode::INTERNAL_SERVER_ERROR, source.to_string()).into_response()
            }
            Error::GameNotInProgress => {
                (StatusCode::BAD_REQUEST, "Game not in progress.").into_response()
            }
            Error::GameNotStartedYet => {
                (StatusCode::BAD_REQUEST, "Game not started yet.").into_response()
            }
            Error::UserMismatch => (StatusCode::BAD_REQUEST, "User mismatch.").into_response(),
            Error::GameEnded => (StatusCode::BAD_REQUEST, "Game ended.").into_response(),
            Error::NotEnoughBalance => {
                (StatusCode::BAD_REQUEST, "Not enough balance.").into_response()
            }
            Error::InvalidBalanceChange(_) => {
                (StatusCode::BAD_REQUEST, "Invalid balance change.").into_response()
            }
            Error::InvalidCurrency => {
                (StatusCode::BAD_REQUEST, "Invalid currency.").into_response()
            }
            Error::ThreadError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Thread error.").into_response()
            }
            Error::PasswordComputeError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Password compute error.").into_response()
            }
            Error::InvalidPassword => {
                (StatusCode::BAD_REQUEST, "Invalid password.").into_response()
            }
            Error::GameLongerThanAllowed => {
                (StatusCode::BAD_REQUEST, "Game longer than allowed.").into_response()
            }
            Error::GameShorterThanAllowed => {
                (StatusCode::BAD_REQUEST, "Game shorter than allowed.").into_response()
            }
            Error::GameLengthMustBePositive => {
                (StatusCode::BAD_REQUEST, "Game length must be positive.").into_response()
            }
            Error::EntryFeeMustBePositive => {
                (StatusCode::BAD_REQUEST, "Entry fee must be positive.").into_response()
            }
            Error::UserExists => (StatusCode::BAD_REQUEST, "User already exists.").into_response(),
            Error::UserNotFound => (StatusCode::NOT_FOUND, "User not found.").into_response(),
            Error::UnexpectedError(source) => {
                (StatusCode::INTERNAL_SERVER_ERROR, source.to_string()).into_response()
            }
            Error::SqlError(source) => {
                (StatusCode::INTERNAL_SERVER_ERROR, source.to_string()).into_response()
            }
            Error::InvalidCredentials(source) => {
                (StatusCode::BAD_REQUEST, source.to_string()).into_response()
            }
            Error::InvalidEmail(source) => (StatusCode::BAD_REQUEST, source).into_response(),
            Error::InvalidUserName(source) => (StatusCode::BAD_REQUEST, source).into_response(),
        }
    }
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        tracing::error!("Error occurred: {}", error);
        match error {
            Error::NonceNotFound => StatusCode::NOT_FOUND,
            Error::AuthError(_) => StatusCode::UNAUTHORIZED,
            Error::GameMevLocked => StatusCode::BAD_REQUEST,
            Error::U64MustBePositive => StatusCode::INTERNAL_SERVER_ERROR,
            Error::GameStatusUnknown => StatusCode::INTERNAL_SERVER_ERROR,
            Error::GameAlreadyStarted => StatusCode::BAD_REQUEST,
            Error::ParsePubkeyError(_) => StatusCode::BAD_REQUEST,
            Error::BadFloat(_) => StatusCode::BAD_REQUEST,
            Error::GasTooLow => StatusCode::BAD_REQUEST,
            Error::GetBalanceError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::FailedToCastConfigToKeyValue(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ConfigNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotOnlyPlayerInGame => StatusCode::BAD_REQUEST,
            Error::BadEnvironmentVariable => StatusCode::INTERNAL_SERVER_ERROR,
            Error::SignatureVerificationError(_) => StatusCode::BAD_REQUEST,
            Error::TransactionVerificationError(_) => StatusCode::BAD_REQUEST,
            Error::GameStillWaitingForOthersToStart => StatusCode::BAD_REQUEST,
            Error::RandomGeneratorError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::GameNotInProgress => StatusCode::BAD_REQUEST,
            Error::GameNotStartedYet => StatusCode::BAD_REQUEST,
            Error::UserMismatch => StatusCode::BAD_REQUEST,
            Error::GameEnded => StatusCode::BAD_REQUEST,
            Error::NotEnoughBalance => StatusCode::BAD_REQUEST,
            Error::InvalidBalanceChange(_) => StatusCode::BAD_REQUEST,
            Error::InvalidCurrency => StatusCode::BAD_REQUEST,
            Error::ThreadError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::PasswordComputeError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidPassword => StatusCode::BAD_REQUEST,
            Error::GameLongerThanAllowed => StatusCode::BAD_REQUEST,
            Error::GameShorterThanAllowed => StatusCode::BAD_REQUEST,
            Error::GameLengthMustBePositive => StatusCode::BAD_REQUEST,
            Error::EntryFeeMustBePositive => StatusCode::BAD_REQUEST,
            Error::UserExists => StatusCode::BAD_REQUEST,
            Error::UserNotFound => StatusCode::NOT_FOUND,
            Error::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::SqlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidCredentials(_) => StatusCode::BAD_REQUEST,
            Error::InvalidEmail(_) => StatusCode::BAD_REQUEST,
            Error::InvalidUserName(_) => StatusCode::BAD_REQUEST,
        }
    }
}
