use chrono::prelude::*;
use super::account::Account;
use super::status::Status;

#[derive(Debug, Clone, Deserialize)]
pub struct Notification {
    pub id: u64,
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    pub created_at: DateTime<UTC>,
    pub account: Account,
    pub status: Option<Status>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum NotificationType {
    #[serde(rename = "mention")]
    Mention,
    #[serde(rename = "reblog")]
    Reblog,
    #[serde(rename = "favourite")]
    Favourite,
    #[serde(rename = "follow")]
    Follow,
}
