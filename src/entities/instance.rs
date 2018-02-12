//! Module containing everything related to an instance.

/// A struct containing info of an instance.
#[derive(Debug, Clone, Deserialize)]
pub struct Instance {
    /// URI of the current instance
    pub uri: String,
    /// The instance's title.
    pub title: String,
    /// A description for the instance.
    pub description: String,
    /// An email address which can be used to contact the
    /// instance administrator.
    pub email: String,
    /// The Mastodon version used by instance.
    pub version: String,
    /// `streaming_api`
    pub urls: Vec<String>,
}
