#[derive(Debug, Default, Clone, Serialize)]
pub struct StatusBuilder {
    status: String,
    #[serde(skip_serializing_if="Option::is_none")]
    in_reply_to_id: Option<u64>,
    #[serde(skip_serializing_if="Option::is_none")]
    media_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if="Option::is_none")]
    sensitive: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    spoiler_text: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    visibility: Option<Visibility>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Visibility {
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "unlisted")]
    Unlisted,
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
