pub mod account;
pub mod attachment;
pub mod card;
pub mod context;
pub mod instance;
pub mod notification;
pub mod relationship;
pub mod report;
pub mod search_result;
pub mod status;

/// An empty JSON object.
#[derive(Deserialize)]
pub struct Empty {}

pub mod prelude {
    pub use super::Empty;
    pub use super::account::Account;
    pub use super::attachment::{Attachment, MediaType};
    pub use super::card::Card;
    pub use super::context::Context;
    pub use super::instance::Instance;
    pub use super::notification::Notification;
    pub use super::relationship::Relationship;
    pub use super::report::Report;
    pub use super::search_result::SearchResult;
    pub use super::status::{Status, Application};
}
