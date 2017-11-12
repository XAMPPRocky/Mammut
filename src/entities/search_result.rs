//! A module containing info relating to a search result.

use super::prelude::{Account, Status};

/// A struct containing results of a search.
#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult {
    /// An array of matched Accounts.
    pub accounts: Vec<Account>,
    /// An array of matched Statuses.
    pub statuses: Vec<Status>,
    /// An array of matched hashtags, as strings.
    pub hashtags: Vec<String>,
}
