//! A module about contexts of statuses.

use super::status::Status;

/// A context of a status returning a list of statuses it replied to and
/// statuses replied to it.
#[derive(Debug, Clone, Deserialize)]
pub struct Context {
    /// Statuses that were replied to.
    pub ancestors: Vec<Status>,
    /// Statuses that replied to this status.
    pub descendants: Vec<Status>,
}
