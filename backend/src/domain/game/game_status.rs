use serde::{Deserialize, Serialize};

use brain_wars::state::game::GameStatus as OnChainGameStatus;
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::{Debug, Display};
use typeshare::typeshare;

#[typeshare]
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum GameStatus {
    WaitingForPlayers,
    OnGoing,
    Finished,
    Settled,
}

impl GameStatus {
    pub fn on_chain_to_db(value: &OnChainGameStatus) -> GameStatus {
        match value {
            OnChainGameStatus::WaitingForPlayers => GameStatus::WaitingForPlayers,
            OnChainGameStatus::OnGoing => GameStatus::OnGoing,
            OnChainGameStatus::Finished => GameStatus::Finished,
            OnChainGameStatus::Settled => GameStatus::Settled,
        }
    }
}

impl sqlx::Type<Postgres> for GameStatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for GameStatus {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for GameStatus {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}

impl Display for GameStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            GameStatus::WaitingForPlayers => "WaitingForPlayers".to_string(),
            GameStatus::OnGoing => "OnGoing".to_string(),
            GameStatus::Finished => "Finished".to_string(),
            GameStatus::Settled => "Settled".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<String> for GameStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "WaitingForPlayers" => GameStatus::WaitingForPlayers,
            "OnGoing" => GameStatus::OnGoing,
            "Finished" => GameStatus::Finished,
            "Settled" => GameStatus::Settled,
            _ => GameStatus::WaitingForPlayers,
        }
    }
}
