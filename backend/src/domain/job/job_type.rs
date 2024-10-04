use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum JobType {
    JoinGame,
    StartGame,
    LeaveGame,
    FinishGame,
    SettleGame,
    Invalid,
}

impl Display for JobType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobType::JoinGame => write!(f, "JoinGame"),
            JobType::StartGame => write!(f, "StartGame"),
            JobType::LeaveGame => write!(f, "LeaveGame"),
            JobType::FinishGame => write!(f, "FinishGame"),
            JobType::SettleGame => write!(f, "SettleGame"),
            JobType::Invalid => write!(f, "Invalid"),
        }
    }
}

impl From<String> for JobType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "JoinGame" => JobType::JoinGame,
            "LeaveGame" => JobType::LeaveGame,
            "StartGame" => JobType::StartGame,
            "FinishGame" => JobType::FinishGame,
            "SettleGame" => JobType::SettleGame,
            _ => JobType::Invalid,
        }
    }
}

impl sqlx::Type<Postgres> for JobType {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for JobType {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for JobType {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}
