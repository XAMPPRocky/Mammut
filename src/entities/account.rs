#![allow(dead_code)]
//! A module containing everything relating to a account returned from the api.

use chrono::prelude::*;

/// A struct representing an Account.
#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    /// Equals `username` for local users, includes `@domain` for remote ones.
    pub acct: String,
    /// URL to the avatar image
    pub avatar: String,
    /// URL to the avatar static image (gif)
    pub avatar_static: String,
    /// The time the account was created.
    pub created_at: DateTime<Utc>,
    /// The account's display name.
    pub display_name: String,
    /// The number of followers for the account.
    pub followers_count: u64,
    /// The number of accounts the given account is following.
    pub following_count: u64,
    /// URL to the header image.
    pub header: String,
    /// URL to the header static image (gif).
    pub header_static: String,
    /// The ID of the account.
    // The ID is transmitted as string type, but it is really an integer.
    // Convert it with code copied from
    // https://github.com/serde-rs/json/issues/329#issuecomment-343535627
    #[serde(with = "string_or_int")]
    pub id: u64,
    /// Boolean for when the account cannot be followed without waiting for
    /// approval first.
    pub locked: bool,
    /// Biography of user.
    pub note: String,
    /// The number of statuses the account has made.
    pub statuses_count: u64,
    /// URL of the user's profile page (can be remote).
    pub url: String,
    /// The username of the account.
    pub username: String,
}

// Accept String or u64 from JSON.
mod string_or_int {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrInt {
            String(String),
            Int(u64),
        }

        match StringOrInt::deserialize(deserializer)? {
            StringOrInt::String(s) => s.parse().map_err(de::Error::custom),
            StringOrInt::Int(i) => Ok(i),
        }
    }
}
