//! A module containing everything relating to a account returned from the api.

use chrono::prelude::*;
use super::string_or_int;

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

mod tests {
    extern crate serde_json as json;
    use entities::account::Account;

    #[test]
    fn account_valid_int_id() {
        let account_json = r#" {
            "id": "123456",
            "username": "example",
            "acct": "example",
            "display_name": "example",
            "locked": false,
            "created_at": "2017-01-01T21:04:21.054Z",
            "note": "\u003cp\u003esome info \u003ca href=\"https://example.com/tags/rustlang\" class=\"mention hashtag\" rel=\"tag\"\u003e#\u003cspan\u003erustlang\u003c/span\u003e\u003c/a\u003e\u003c/p\u003e",
            "url": "https://example.com/@example",
            "avatar": "https://files.example.com/accounts/avatars/000/028/407/original/abcdef.png",
            "avatar_static": "https://files.example.com/accounts/avatars/000/028/407/original/1234.png",
            "header": "https://example.com/headers/original/missing.png",
            "header_static": "https://example.com/headers/original/missing.png",
            "followers_count": 25,
            "following_count": 44,
            "statuses_count": 33,
            "source": {
                "privacy": "public",
                "sensitive": false,
                "note": "some note"
            }
        } "#;
        let account: Account = json::from_str(account_json).unwrap();
        assert_eq!(account.id, 123456);
    }

    #[test]
    fn account_valid_string_id() {
        let account_json = r#" {
            "id": 123456,
            "username": "example",
            "acct": "example",
            "display_name": "example",
            "locked": false,
            "created_at": "2017-01-01T21:04:21.054Z",
            "note": "\u003cp\u003esome info \u003ca href=\"https://example.com/tags/rustlang\" class=\"mention hashtag\" rel=\"tag\"\u003e#\u003cspan\u003erustlang\u003c/span\u003e\u003c/a\u003e\u003c/p\u003e",
            "url": "https://example.com/@example",
            "avatar": "https://files.example.com/accounts/avatars/000/028/407/original/abcdef.png",
            "avatar_static": "https://files.example.com/accounts/avatars/000/028/407/original/1234.png",
            "header": "https://example.com/headers/original/missing.png",
            "header_static": "https://example.com/headers/original/missing.png",
            "followers_count": 25,
            "following_count": 44,
            "statuses_count": 33,
            "source": {
                "privacy": "public",
                "sensitive": false,
                "note": "some note"
            }
        } "#;
        let account: Account = json::from_str(account_json).unwrap();
        assert_eq!(account.id, 123456);
    }
}
