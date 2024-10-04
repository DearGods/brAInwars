use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::str::FromStr;
use typeshare::typeshare;
use uuid::Uuid;

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct Id(#[typeshare(serialized_as = "string")] Uuid);

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl Id {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Self(Uuid::parse_str(&s).unwrap())
    }
}

impl From<&[u8]> for Id {
    fn from(s: &[u8]) -> Self {
        Self(Uuid::parse_str(std::str::from_utf8(s).unwrap()).unwrap())
    }
}

impl FromStr for Id {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Id> for Uuid {
    fn from(id: Id) -> Self {
        id.0
    }
}

impl From<Id> for String {
    fn from(id: Id) -> Self {
        id.0.to_string()
    }
}

impl From<Uuid> for Id {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl AsRef<Uuid> for Id {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl sqlx::Type<Postgres> for Id {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <Uuid as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for Id {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <Uuid as sqlx::Encode<Postgres>>::encode(self.0, buf)
    }
}

impl sqlx::Decode<'_, Postgres> for Id {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        Ok(Self::from_str(value)?)
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Uuid as std::fmt::Display>::fmt(&self.0, f)
    }
}

impl PartialEq<Uuid> for Id {
    fn eq(&self, other: &Uuid) -> bool {
        self.0 == *other
    }
}
