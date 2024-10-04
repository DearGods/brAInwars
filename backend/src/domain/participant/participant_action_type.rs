use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum ParticipantActionType {
    JOIN,
    LEAVE,
    JOINED_LEFT_BEFORE_START,
}

impl sqlx::Type<Postgres> for ParticipantActionType {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Decode<'_, Postgres> for ParticipantActionType {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = ParticipantActionType::from_str(value)?;
        Ok(value)
    }
}

impl FromStr for ParticipantActionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JOIN" => Ok(ParticipantActionType::JOIN),
            "LEAVE" => Ok(ParticipantActionType::LEAVE),
            "JOINED_LEFT_BEFORE_START" => Ok(ParticipantActionType::JOINED_LEFT_BEFORE_START),
            _ => Err("Invalid participant action type".to_string()),
        }
    }
}

impl ToString for ParticipantActionType {
    fn to_string(&self) -> String {
        match self {
            ParticipantActionType::JOIN => "JOIN".to_string(),
            ParticipantActionType::LEAVE => "LEAVE".to_string(),
            ParticipantActionType::JOINED_LEFT_BEFORE_START => {
                "JOINED_LEFT_BEFORE_START".to_string()
            }
        }
    }
}
