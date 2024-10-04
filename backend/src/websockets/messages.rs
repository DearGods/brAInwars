use crate::domain::id::Id;
use crate::startup::application::AppState;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::sync::Arc;
use typeshare::typeshare;

#[tracing::instrument(skip(app_state))]
pub fn publish_msg(app_state: Arc<AppState>, message: String) {
    let _ = app_state.game_tx.send(message);
}

#[typeshare]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum MsgName {
    GAME_COUNTER,
    NEW_GAME_CREATED,
    GAME_SETTLED,
    GAME_ENDED,
    GAME_STARTED,
    PLAYER_JOINED,
    PLAYER_LEFT,
}

impl Display for MsgName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::GAME_COUNTER => "GAME_COUNTER".to_string(),
            Self::GAME_SETTLED => "GAME_SETTLED".to_string(),
            Self::NEW_GAME_CREATED => "NEW_GAME_CREATED".to_string(),
            Self::GAME_ENDED => "GAME_ENDED".to_string(),
            Self::GAME_STARTED => "GAME_STARTED".to_string(),
            Self::PLAYER_JOINED => "PLAYER_JOINED".to_string(),
            Self::PLAYER_LEFT => "PLAYER_LEFT".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerJoined {
    pub msg_name: MsgName,
    pub game_id: Id,
    pub wallet_address: String,
    pub name: String,
    pub msg_id: Id,
}

impl Display for PlayerJoined {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerLeft {
    pub msg_name: MsgName,
    pub game_id: Id,
    pub wallet_address: String,
    pub name: String,
    pub msg_id: Id,
}

impl Display for PlayerLeft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct GameStarted {
    pub msg_name: MsgName,
    pub game_id: Id,
    pub msg_id: Id,
}

impl Display for GameStarted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct NewGameCreated {
    pub msg_name: MsgName,
    pub game_id: Id,
    pub msg_id: Id,
}

impl Display for NewGameCreated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct GameEnded {
    pub msg_name: MsgName,
    pub game_id: Id,
    pub msg_id: Id,
    pub winner: Option<String>,
}

impl Display for GameEnded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct GameSettled {
    pub msg_name: MsgName,
    pub game_id: Id,
    pub msg_id: Id,
}

impl Display for GameSettled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct GameCounter {
    pub msg_name: MsgName,
    pub game_id: Id,
    pub msg_id: Id,
    pub start_time_from_now_in_milliseconds: f64,
}

impl Display for GameCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
