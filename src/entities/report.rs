//! module containing information about a finished report of a user.

/// A struct containing info about a report.
#[derive(Debug, Clone, Deserialize)]
pub struct Report {
    /// The ID of the report.
    pub id: String,
    /// The action taken in response to the report.
    pub action_taken: String,
}
