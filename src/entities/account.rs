#![allow(dead_code)]
use chrono::prelude::*;
#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    // The ID is transmitted as string type, but it is really an integer.
    // Convert it with code copied from
    // https://github.com/serde-rs/json/issues/329#issuecomment-305608405
    #[serde(with = "string")] pub id: u64,
    pub username: String,
    pub acct: String,
    pub display_name: String,
    pub note: String,
    pub url: String,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
    pub locked: bool,
    pub created_at: DateTime<Utc>,
    pub followers_count: u64,
    pub following_count: u64,
    pub statuses_count: u64,
}

mod string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}
