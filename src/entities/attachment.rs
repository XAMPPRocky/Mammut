#[derive(Debug, Clone, Deserialize)]
pub struct Attachment {
    pub id: u64,
    #[serde(rename="type")]
    pub media_type: MediaType,
    pub url: String,
    pub remote_url: String,
    pub preview_url: String,
    pub text_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum MediaType {
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "gifv")]
    Gifv,
}
