//! Module containing everything related to media attachements.
use super::Empty;
use serde::{Deserialize, Deserializer};

/// A struct representing a media attachment.
#[derive(Debug, Clone, Deserialize)]
pub struct Attachment {
    /// ID of the attachment.
    pub id: String,
    /// The media type of an attachment.
    #[serde(rename = "type")]
    pub media_type: MediaType,
    /// URL of the locally hosted version of the image.
    pub url: String,
    /// For remote images, the remote URL of the original image.
    pub remote_url: Option<String>,
    /// URL of the preview image.
    pub preview_url: String,
    /// Shorter URL for the image, for insertion into text
    /// (only present on local images)
    pub text_url: Option<String>,
    /// Meta information about the attachment.
    #[serde(deserialize_with = "empty_as_none")]
    pub meta: Option<Meta>,
    /// Noop will be removed.
    pub description: Option<String>,
}

fn empty_as_none<'de, D: Deserializer<'de>>(val: D) -> Result<Option<Meta>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum EmptyOrMeta {
        Empty(Empty),
        Meta(Meta),
    }

    Ok(match EmptyOrMeta::deserialize(val)? {
        EmptyOrMeta::Empty(_) => None,
        EmptyOrMeta::Meta(m) => Some(m),
    })
}

/// Information about the attachment itself.
#[derive(Debug, Deserialize, Clone)]
pub struct Meta {
    /// Original version.
    original: ImageDetails,
    /// Smaller version.
    small: ImageDetails,
}

/// Dimensions of an attachement.
#[derive(Debug, Deserialize, Clone)]
pub struct ImageDetails {
    /// width of attachment.
    width: u64,
    /// height of attachment.
    height: u64,
    /// A string of `widthxheight`.
    size: String,
    /// The aspect ratio of the attachment.
    aspect: f64,
}

/// The type of media attachment.
#[derive(Debug, Deserialize, Clone, Copy)]
pub enum MediaType {
    /// An image.
    #[serde(rename = "image")]
    Image,
    /// A video file.
    #[serde(rename = "video")]
    Video,
    /// A gifv format file.
    #[serde(rename = "gifv")]
    Gifv,
    /// Unknown format.
    #[serde(rename = "unknown")]
    Unknown,
}
