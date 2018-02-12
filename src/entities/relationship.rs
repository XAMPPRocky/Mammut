//! module containing everything relating to a relationship with
//! another account.

/// A struct containing information about a relationship with another account.
#[derive(Debug, Clone, Deserialize)]
pub struct Relationship {
    /// Target account id
    pub id: String,
    /// Whether the application client follows the account.
    pub following: bool,
    /// Whether the account follows the application client.
    pub followed_by: bool,
    /// Whether the application client blocks the account.
    pub blocking: bool,
    /// Whether the application client blocks the account.
    pub muting: bool,
    /// Whether the application client has requested to follow the account.
    pub requested: bool,
    /// Whether the user is also muting notifications
    pub muting_notifications: bool,
    /// Whether the user is currently blocking the accounts's domain
    pub domain_blocking: bool,
}
