/// A builder pattern struct for constructing a status.
#[derive(Debug, Default, Clone, Serialize)]
pub struct StatusBuilder {
    /// The text of the status.
    pub status: String,
    /// Ids of accounts being replied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<u64>,
    /// Ids of media attachments being attached to the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_ids: Option<Vec<u64>>,
    /// Whether current status is sensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    /// Text to precede the normal status text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoiler_text: Option<String>,
    /// Visibility of the status, defaults to `Public`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

/// The visibility of a status.
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
    /// Create a new status with text.
    /// ```
    /// let status = StatusBuilder::new("Hello World!".into());
    /// ```
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
