use chrono::prelude::*;
use super::account::Account;
use super::status::Status;

#[derive(Deserialize)]
pub struct Notification {
    pub id: u64,
    pub notification_type: NotificationType,
    pub created_at: DateTime<UTC>,
    pub account: Account,
    pub status: Option<Status>,
}

#[derive(Deserialize)]
pub enum NotificationType {
    Mention,
    Reblog,
    Favourite,
    Follow,
}
