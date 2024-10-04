use crate::errors::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::ops::Add;

#[derive(Deserialize, Serialize, Debug, Clone, Default, Copy, PartialEq)]
pub struct U64(u64);

impl U64 {
    #[tracing::instrument(name = "validate U64", level = "trace", ret, err)]
    pub fn new(i: i64) -> Result<Self, Error> {
        if i < 0i64 {
            return Err(Error::U64MustBePositive);
        }
        Ok(U64(i as u64))
    }

    pub fn get(&self) -> u64 {
        self.0
    }

    pub fn to_i64(&self) -> i64 {
        self.0 as i64
    }

    pub fn convert_i64_option(v: &Option<U64>) -> Option<i64> {
        v.as_ref().map(|v| v.to_i64())
    }
}

impl Add for U64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<u64> for U64 {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl From<&u64> for U64 {
    fn from(i: &u64) -> Self {
        Self(*i)
    }
}

impl From<u64> for U64 {
    fn from(i: u64) -> Self {
        Self(i)
    }
}

impl TryFrom<i64> for U64 {
    type Error = Error;

    fn try_from(i: i64) -> Result<Self, Self::Error> {
        Self::new(i)
    }
}

impl AsRef<u64> for U64 {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl sqlx::Type<Postgres> for U64 {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <i64 as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Decode<'_, Postgres> for U64 {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <i64 as Decode<Postgres>>::decode(value)?;
        Ok(U64::new(value)?)
    }
}

impl sqlx::Encode<'_, Postgres> for U64 {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <i64 as sqlx::Encode<Postgres>>::encode(self.0 as i64, buf)
    }
}
