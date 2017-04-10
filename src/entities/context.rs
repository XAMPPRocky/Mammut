use super::status::Status;

#[derive(Deserialize)]
pub struct Context {
    pub ancestors: Vec<Status>,
    pub descendants: Vec<Status>,
}
