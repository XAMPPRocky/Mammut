use super::{Mastodon, Result, deserialise};
use regex::Regex;
use reqwest::Response;
use reqwest::header::{LINK};
use serde::Deserialize;
use url::Url;
use entities::itemsiter::ItemsIter;

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

    // @todo Remove the hackish regexes once the reqwest library supports typed
    // headers again.
    if let Some(link_header) = response.headers().get(LINK) {
        let header_string = link_header.to_str()?;
        let regex = Regex::new("<([^>]+)>; rel=\"prev\"").unwrap();
        if let Some(captures) = regex.captures(header_string) {
            // The unwrap() is ok here because we know that if we have a match
            // then capture 1 must be filled.
            prev = Some(Url::parse(captures.get(1).unwrap().as_str())?);
        }
        let regex = Regex::new("<([^>]+)>; rel=\"next\"").unwrap();
        if let Some(captures) = regex.captures(header_string) {
            // The unwrap() is ok here because we know that if we have a match
            // then capture 1 must be filled.
            next = Some(Url::parse(captures.get(1).unwrap().as_str())?);
        }
    }

    Ok((prev, next))
}

#[cfg(test)]
mod tests {
    extern crate http;
    use self::http::Response;
    use reqwest::header::{LINK};
    use page::get_links;
    use url::Url;

    #[test]
    fn get_prev_next() {
        let mut response = Response::builder();
        response.header(LINK, "<https://example.com/api/v1/accounts/1234/statuses?exclude_replies=1&max_id=100912162655190397>; rel=\"next\", <https://example.com/api/v1/accounts/1234/statuses?exclude_replies=1&min_id=101008854461266530>; rel=\"prev\"");
        let links = get_links(&response.body("").unwrap().into()).unwrap();
        assert_eq!(links.0, Some(Url::parse("https://example.com/api/v1/accounts/1234/statuses?exclude_replies=1&min_id=101008854461266530").unwrap()));
        assert_eq!(links.1, Some(Url::parse("https://example.com/api/v1/accounts/1234/statuses?exclude_replies=1&max_id=100912162655190397").unwrap()));
    }
}
