use super::{deserialise, Mastodon, Result};
use reqwest::Response;
use reqwest::header::{Link, RelationType};
use serde::Deserialize;
use url::Url;

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
            mastodon,
        })
    }

    pages! {
        next: next_page,
        prev: prev_page
    }
}

fn get_links(response: &Response) -> Result<(Option<Url>, Option<Url>)> {
    let mut prev = None;
    let mut next = None;

    if let Some(link_header) = response.headers().get::<Link>() {
        for value in link_header.values() {
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
