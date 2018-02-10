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
    pub id: String,
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
