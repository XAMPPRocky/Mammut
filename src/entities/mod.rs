pub mod account;
pub mod attachment;
pub mod card;
pub mod context;
pub mod instance;
pub mod notification;
pub mod relationship;
pub mod report;
pub mod search_result;
pub mod status;

/// An empty JSON object.
#[derive(Deserialize)]
pub struct Empty {}

pub mod prelude {
    //! The purpose of this module is to alleviate imports of many common structs
    //! by adding a glob import to the top of mastodon heavy modules:
    pub use super::Empty;
    pub use super::account::Account;
    pub use super::attachment::{Attachment, MediaType};
    pub use super::card::Card;
    pub use super::context::Context;
    pub use super::instance::Instance;
    pub use super::notification::Notification;
    pub use super::relationship::Relationship;
    pub use super::report::Report;
    pub use super::search_result::SearchResult;
    pub use super::status::{Application, Status};
}

// Accept String or u64 from JSON.
#[allow(dead_code)]
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

// Accept String or u64 or null from JSON.
#[allow(dead_code)]
mod option_string_or_int {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrIntOrNull {
            String(String),
            Int(u64),
            None,
        }

        match StringOrIntOrNull::deserialize(deserializer)? {
            StringOrIntOrNull::String(s) => {
                let parsed: Result<u64, _> = s.parse().map_err(de::Error::custom);
                match parsed {
                    Ok(value) => Ok(Some(value)),
                    Err(error) => Err(error),
                }
            }
            StringOrIntOrNull::Int(i) => Ok(Some(i)),
            StringOrIntOrNull::None => Ok(None),
        }
    }
}
