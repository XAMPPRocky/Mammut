//! Module containing all info about notifications.

use chrono::prelude::*;
use super::account::Account;
use super::status::Status;

/// A struct containing info about a notification.
#[derive(Debug, Clone, Deserialize)]
pub struct Notification {
    /// The notification ID.
    pub id: String,
    /// The type of notification.
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    /// The time the notification was created.
    pub created_at: DateTime<Utc>,
    /// The Account sending the notification to the user.
    pub account: Account,
    /// The Status associated with the notification, if applicable.
    pub status: Option<Status>,
}

/// The type of notification.
#[derive(Debug, Clone, Deserialize)]
pub enum NotificationType {
    /// Someone mentioned the application client in another status.
    #[serde(rename = "mention")]
    Mention,
    /// Someone reblogged one of the application client's statuses.
    #[serde(rename = "reblog")]
    Reblog,
    /// Someone favourited one of the application client's statuses.
    #[serde(rename = "favourite")]
    Favourite,
    /// Someone followed the application client.
    #[serde(rename = "follow")]
    Follow,
}
