use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Date(#[typeshare(serialized_as = "string")] DateTime<Utc>);

impl From<DateTime<Utc>> for Date {
    fn from(date: DateTime<Utc>) -> Self {
        Self(date)
    }
}

impl PartialEq<DateTime<Utc>> for Date {
    fn eq(&self, other: &DateTime<Utc>) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<DateTime<Utc>> for Date {
    fn partial_cmp(&self, other: &DateTime<Utc>) -> Option<Ordering> {
        Some(self.0.cmp(other))
    }
}
