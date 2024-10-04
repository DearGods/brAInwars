use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_camel_case_types)]
pub enum JobStatus {
    Pending,
    Running,
    Error,
    Done,
    Invalid,
}

impl Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Pending => write!(f, "Pending"),
            JobStatus::Running => write!(f, "Running"),
            JobStatus::Done => write!(f, "Done"),
            JobStatus::Error => write!(f, "Error"),
            JobStatus::Invalid => write!(f, "Invalid"),
        }
    }
}

impl From<String> for JobStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Running" => JobStatus::Running,
            "Pending" => JobStatus::Pending,
            "Done" => JobStatus::Done,
            "Error" => JobStatus::Error,
            _ => JobStatus::Invalid,
        }
    }
}

impl sqlx::Type<Postgres> for JobStatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for JobStatus {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for JobStatus {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}
