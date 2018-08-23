use std::borrow::Cow;

/// # Example
///
/// ```
/// # extern crate elefren;
/// # use elefren::StatusesRequest;
/// let request = StatusesRequest::new()
///                               .only_media()
///                               .pinned()
///                               .since_id("foo");
/// # assert_eq!(&request.to_querystring()[..], "?only_media=1&pinned=1&since_id=foo");
/// ```
#[derive(Clone, Debug, Default)]
pub struct StatusesRequest<'a> {
    only_media: bool,
    exclude_replies: bool,
    pinned: bool,
    max_id: Option<Cow<'a, str>>,
    since_id: Option<Cow<'a, str>>,
    limit: Option<usize>,
}

impl<'a> StatusesRequest<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn only_media(mut self) -> Self {
        self.only_media = true;
        self
    }

    pub fn exclude_replies(mut self) -> Self {
        self.exclude_replies = true;
        self
    }

    pub fn pinned(mut self) -> Self {
        self.pinned = true;
        self
    }

    pub fn max_id<S: Into<Cow<'a, str>>>(mut self, max_id: S) -> Self {
        self.max_id = Some(max_id.into());
        self
    }

    pub fn since_id<S: Into<Cow<'a, str>>>(mut self, since_id: S) -> Self {
        self.since_id = Some(since_id.into());
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn to_querystring(&self) -> String {
        let mut opts = vec![];

        if self.only_media {
            opts.push("only_media=1".into());
        }

        if self.exclude_replies {
            opts.push("exclude_replies=1".into());
        }

        if self.pinned {
            opts.push("pinned=1".into());
        }

        if let Some(ref max_id) = self.max_id {
            opts.push(format!("max_id={}", max_id));
        }

        if let Some(ref since_id) = self.since_id {
            opts.push(format!("since_id={}", since_id));
        }

        if let Some(limit) = self.limit {
            opts.push(format!("limit={}", limit));
        }

        if opts.is_empty() {
            String::new()
        } else {
            format!("?{}", opts.join("&"))
        }
    }
}


