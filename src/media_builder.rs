use std::borrow::Cow;

/// A builder pattern struct for constructing a media attachment.
#[derive(Debug, Default, Clone, Serialize)]
pub struct MediaBuilder {
    /// The file name of the attachment to be uploaded.
    pub file: Cow<'static, str>,
    /// The alt text of the attachment.
    pub description: Option<Cow<'static, str>>,
    /// The focus point for images.
    pub focus: Option<(f32, f32)>,
}

impl MediaBuilder {
    /// Create a new attachment from a file name.
    pub fn new(file: Cow<'static, str>) -> Self {
        MediaBuilder {
            file,
            description: None,
            focus: None,
        }
    }
    /// Set an alt text description for the attachment.
    pub fn description(mut self, description: Cow<'static, str>) -> Self {
        self.description = Some(description);
        self
    }

    /// Set a focus point for an image attachment.
    pub fn focus(mut self, f1: f32, f2: f32) -> Self {
        self.focus = Some((f1, f2));
        self
    }
}

// Convenience helper so that the mastodon.media() method can be called with a
// file name only (owned string).
impl From<String> for MediaBuilder {
    fn from(file: String) -> MediaBuilder {
        MediaBuilder {
            file: file.into(),
            description: None,
            focus: None,
        }
    }
}

// Convenience helper so that the mastodon.media() method can be called with a
// file name only (borrowed string).
impl From<&'static str> for MediaBuilder {
    fn from(file: &'static str) -> MediaBuilder {
        MediaBuilder {
            file: file.into(),
            description: None,
            focus: None,
        }
    }
}

// Convenience helper so that the mastodon.media() method can be called with a
// file name only (Cow string).
impl From<Cow<'static, str>> for MediaBuilder {
    fn from(file: Cow<'static, str>) -> MediaBuilder {
        MediaBuilder {
            file,
            description: None,
            focus: None,
        }
    }
}
