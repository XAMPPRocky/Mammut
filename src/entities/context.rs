use super::status::Status;

#[derive(Debug, Clone, Deserialize)]
pub struct Context {
    pub ancestors: Vec<Status>,
    pub descendants: Vec<Status>,
}
