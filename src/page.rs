use reqwest::Response;
use reqwest::header::LINK;
use hyperx::header::{Header, Link, RelationType};
use serde::Deserialize;
use url::Url;

use crate::entities::itemsiter::ItemsIter;
use super::{Mastodon, Result, deserialise};

pub struct Page<'a, T: for<'de> Deserialize<'de>> {
    mastodon: &'a Mastodon,
    next: Option<Url>,
    prev: Option<Url>,
    /// Initial set of items
    pub initial_items: Vec<T>,
}

macro_rules! pages {
    ($($direction:ident: $fun:ident),*) => {

        $(
            pub fn $fun(&mut self) -> Result<Option<Vec<T>>> {
                let url = match self.$direction.take() {
                    Some(s) => s,
                    None => return Ok(None),
                };

                let response = self.mastodon.client.get(url)
                    .headers(self.mastodon.headers.clone())
                    .send()?;

                let (prev, next) = get_links(&response)?;
                self.next = next;
                self.prev = prev;

                deserialise(response)
            }
         )*
    }
}

impl<'a, T: for<'de> Deserialize<'de>> Page<'a, T> {
    pub fn new(mastodon: &'a Mastodon, response: Response) -> Result<Self> {
        let (prev, next) = get_links(&response)?;
        Ok(Page {
            initial_items: deserialise(response)?,
            next,
            prev,
            mastodon
        })
    }

    pages! {
        next: next_page,
        prev: prev_page
    }
}

impl<'a, T: Clone + for<'de> Deserialize<'de>> Page<'a, T> {
    /// Returns an iterator that provides a stream of `T`s
    ///
    /// This abstracts away the process of iterating over each item in a page, then making an http
    /// call, then iterating over each item in the new page, etc. The iterator provides a stream of
    /// `T`s, calling `self.next_page()` when necessary to get more of them, until there are no more
    /// items.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # extern crate mammut;
    /// # use std::error::Error;
    /// use mammut::{Mastodon, Data, StatusesRequest};
    /// # fn main() -> Result<(), Box<Error>> {
    /// #   let data = Data {
    /// #       base: "".into(),
    /// #       client_id: "".into(),
    /// #       client_secret: "".into(),
    /// #       redirect: "".into(),
    /// #       token: "".into(),
    /// #   };
    /// let mastodon = Mastodon::from_data(data);
    /// let req = StatusesRequest::new();
    /// let resp = mastodon.statuses("some-id", req)?;
    /// for status in resp.items_iter() {
    ///     // do something with status
    /// }
    /// #   Ok(())
    /// # }
    /// ```
    pub fn items_iter(self) -> impl Iterator<Item = T> + 'a
            where T: 'a
    {
        ItemsIter::new(self)
    }
}

fn get_links(response: &Response) -> Result<(Option<Url>, Option<Url>)> {
    let mut prev = None;
    let mut next = None;

    let link_header = response.headers().get_all(LINK);
    for value in &link_header {
        let parsed: Link = Header::parse_header(&value)?;
        for value in parsed.values() {
            if let Some(relations) = value.rel() {
                if relations.contains(&RelationType::Next) {
                    next = Some(Url::parse(value.link())?);
                }

                if relations.contains(&RelationType::Prev) {
                    prev = Some(Url::parse(value.link())?);
                }
            }
        }
    }

    Ok((prev, next))
}
