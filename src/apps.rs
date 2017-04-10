#[derive(Debug, Default, Serialize)]
pub struct AppBuilder<'a> {
    pub client_name: &'a str,
    pub redirect_uris: &'a str,
    pub scopes: Scope,
    #[serde(skip_serializing_if="Option::is_none")]
    pub website: Option<&'a str>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Scope {
    #[serde(rename = "read write follow")]
    All,
    #[serde(rename = "follow")]
    Follow,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "read follow")]
    ReadFollow,
    #[serde(rename = "read write")]
    ReadWrite,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "write follow")]
    WriteFollow,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::Read
    }
}
