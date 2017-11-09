#[derive(Debug, Clone, Deserialize)]
pub struct Attachment {
    pub id: String,
    #[serde(rename="type")]
    pub media_type: MediaType,
    pub url: String,
    pub remote_url: Option<String>,
    pub preview_url: String,
    pub text_url: Option<String>,
    pub meta: Option<Meta>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Meta {
    original: ImageDetails,
    small: ImageDetails,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ImageDetails {
    width: u64,
    height: u64,
    size: String,
    aspect: f64,

}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum MediaType {
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "gifv")]
    Gifv,
    #[serde(rename = "unknown")]
    Unknown,
}
