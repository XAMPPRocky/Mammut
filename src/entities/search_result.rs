use super::prelude::{Account, Status};

#[derive(Deserialize)]
pub struct SearchResult {
    pub accounts: Vec<Account>,
    pub statuses: Vec<Status>,
    pub hashtags: Vec<String>,
}
