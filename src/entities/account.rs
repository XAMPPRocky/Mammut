use chrono::prelude::*;
#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    pub id: u64,
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
    pub created_at: DateTime<UTC>,
    pub followers_count: u64,
    pub following_count: u64,
    pub statuses_count: u64,
}
