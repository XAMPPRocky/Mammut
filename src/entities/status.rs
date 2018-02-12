//! Module containing all info relating to a status.

use chrono::prelude::*;
use super::prelude::*;
use status_builder::Visibility;

/// A status from the instance.
#[derive(Debug, Clone, Deserialize)]
pub struct Status {
    /// The ID of the status.
    pub id: String,
    /// A Fediverse-unique resource ID.
    pub uri: String,
    /// URL to the status page (can be remote)
    pub url: Option<String>,
    /// The Account which posted the status.
    pub account: Account,
    /// The ID of the status this status is replying to, if the status is
    /// a reply.
    pub in_reply_to_id: Option<String>,
    /// The ID of the account this status is replying to, if the status is
    /// a reply.
    pub in_reply_to_account_id: Option<String>,
    /// If this status is a reblogged Status of another User.
    pub reblog: Option<Box<Status>>,
    /// Body of the status; this will contain HTML
    /// (remote HTML already sanitized)
    pub content: String,
    /// The time the status was created.
    pub created_at: DateTime<Utc>,
    /// An array of Emoji
    pub emojis: Vec<Emoji>,
    /// The number of reblogs for the status.
    pub reblogs_count: u64,
    /// The number of favourites for the status.
    pub favourites_count: u64,
    /// Whether the application client has reblogged the status.
    pub reblogged: Option<bool>,
    /// Whether the application client has favourited the status.
    pub favourited: Option<bool>,
    /// Whether media attachments should be hidden by default.
    pub sensitive: bool,
    /// If not empty, warning text that should be displayed before the actual
    /// content.
    pub spoiler_text: String,
    /// The visibilty of the status.
    pub visibility: Visibility,
    /// An array of attachments.
    pub media_attachments: Vec<Attachment>,
    /// An array of mentions.
    pub mentions: Vec<Mention>,
    /// An array of tags.
    pub tags: Vec<Tag>,
    /// Name of application used to post status.
    pub application: Option<Application>,
    /// The detected language for the status, if detected.
    pub language: Option<String>,
    /// Whether this is the pinned status for the account that posted it.
    pub pinned: Option<bool>,
}

/// A mention of another user.
#[derive(Debug, Clone, Deserialize)]
pub struct Mention {
    /// URL of user's profile (can be remote).
    pub url: String,
    /// The username of the account.
    pub username: String,
    /// Equals `username` for local users, includes `@domain` for remote ones.
    pub acct: String,
    /// Account ID.
    pub id: String,
}

/// Struct representing an emoji within text.
#[derive(Clone, Debug, Deserialize)]
pub struct Emoji {
    /// The shortcode of the emoji
    pub shortcode: String,
    /// URL to the emoji static image
    pub static_url: String,
    /// URL to the emoji image
    pub url: String,
}

/// Hashtags in the status.
#[derive(Debug, Clone, Deserialize)]
pub struct Tag {
    /// The hashtag, not including the preceding `#`.
    pub name: String,
    /// The URL of the hashtag.
    pub url: String,
}

/// Application details.
#[derive(Debug, Clone, Deserialize)]
pub struct Application {
    /// Name of the application.
    pub name: String,
    /// Homepage URL of the application.
    pub website: Option<String>,
}
