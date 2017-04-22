use chrono::prelude::*;
use super::prelude::*;
use status_builder::Visibility;

#[derive(Debug, Clone, Deserialize)]
pub struct Status {
    pub id: i64,
    pub uri: String,
    pub url: String,
    pub account: Account,
    pub in_reply_to_id: Option<u64>,
    pub in_reply_to_account_id: Option<u64>,
    pub reblog: Option<Box<Status>>,
    pub content: String,
    pub created_at: DateTime<UTC>,
    pub reblogs_count: u64,
    pub favourites_count: u64,
    pub reblogged: Option<bool>,
    pub favourited: Option<bool>,
    pub sensitive: Option<bool>,
    pub spoiler_text: String,
    pub visibility: Visibility,
    pub media_attachments: Vec<Attachment>,
    pub mentions: Vec<Mention>,
    pub tags: Vec<Tag>,
    pub application: Option<Application>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mention {
    pub url: String,
    pub username: String,
    pub acct: String,
    pub id: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tag {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Application {
    pub name: String,
    pub website: Option<String>,
}
