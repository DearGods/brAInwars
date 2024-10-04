use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum ParticipantStatus {
    JOINED,
    LEFT,
    JOINED_LEFT_BEFORE_START,
}

impl sqlx::Type<Postgres> for ParticipantStatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Decode<'_, Postgres> for ParticipantStatus {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = ParticipantStatus::from_str(value)?;
        Ok(value)
    }
}

impl FromStr for ParticipantStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JOINED" => Ok(ParticipantStatus::JOINED),
            "LEFT" => Ok(ParticipantStatus::LEFT),
            "JOINED_LEFT_BEFORE_START" => Ok(ParticipantStatus::JOINED_LEFT_BEFORE_START),
            _ => Err("Invalid participant status".to_string()),
        }
    }
}

impl ToString for ParticipantStatus {
    fn to_string(&self) -> String {
        match self {
            ParticipantStatus::JOINED => "JOINED".to_string(),
            ParticipantStatus::LEFT => "LEFT".to_string(),
            ParticipantStatus::JOINED_LEFT_BEFORE_START => "JOINED_LEFT_BEFORE_START".to_string(),
        }
    }
}
