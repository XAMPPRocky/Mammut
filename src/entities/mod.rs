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
    //! The purpose of this module is to alleviate imports of many common structs
    //! by adding a glob import to the top of mastodon heavy modules:
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
    pub use super::status::{Application, Status};
}
