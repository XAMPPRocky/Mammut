#[derive(Debug, Clone, Deserialize)]
pub struct Report {
    pub id: u64,
    pub action_taken: String,
}
