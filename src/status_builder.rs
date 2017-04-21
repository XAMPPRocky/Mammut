#[derive(Debug, Default, Clone, Serialize)]
pub struct StatusBuilder {
    status: String,
    /// User ids of those to reply to.
    #[serde(skip_serializing_if="Option::is_none")]
    pub in_reply_to_id: Option<u64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub media_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sensitive: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub spoiler_text: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<Visibility>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Visibility {
    /// A Direct message to a user
    #[serde(rename = "direct")]
    Direct,
    /// Only available to followers
    #[serde(rename = "private")]
    Private,
    /// Not shown in public timelines
    #[serde(rename = "unlisted")]
    Unlisted,
    /// Posted to public timelines
    #[serde(rename = "public")]
    Public,
}

impl StatusBuilder {

    pub fn new(status: String) -> Self {
        StatusBuilder {
            status: status,
            ..Self::default()
        }
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Public
    }
}
