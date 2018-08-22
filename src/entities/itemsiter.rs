use page::Page;
use serde::Deserialize;

/// Abstracts away the `next_page` logic into a single stream of items
///
/// ```no_run
/// # extern crate elefren;
/// # use elefren::prelude::*;
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<Error>> {
/// # let data = Data {
/// #   base: "".into(),
/// #   client_id: "".into(),
/// #   client_secret: "".into(),
/// #   redirect: "".into(),
/// #   token: "".into(),
/// # };
/// let client = Mastodon::from(data);
/// let statuses = client.statuses("user-id", None)?;
/// for status in statuses.items_iter() {
///   // do something with `status`
/// }
/// # Ok(())
/// # }
/// ```
pub(crate) struct ItemsIter<'a, T: Clone + for<'de> Deserialize<'de>> {
    page: Page<'a, T>,
    buffer: Vec<T>,
    cur_idx: usize,
    use_initial: bool,
}

impl<'a, T: Clone + for<'de> Deserialize<'de>> ItemsIter<'a, T> {
    pub(crate) fn new(page: Page<'a, T>) -> ItemsIter<'a, T> {
        ItemsIter {
            page: page,
            buffer: vec![],
            cur_idx: 0,
            use_initial: true,
        }
    }

    fn need_next_page(&self) -> bool {
        self.buffer.is_empty() ||
            self.cur_idx == self.buffer.len()
    }

    fn fill_next_page(&mut self) -> Option<()> {
        let items = if let Ok(items) = self.page.next_page() {
            items
        } else {
            return None;
        };
        if let Some(items) = items {
            self.buffer = items;
            self.cur_idx = 0;
            Some(())
        } else {
            None
        }
    }
}

impl<'a, T: Clone+ for<'de> Deserialize<'de>> Iterator for ItemsIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.use_initial {
            if self.page.initial_items.is_empty() {
                return None;
            }
            let idx = self.cur_idx;
            if self.cur_idx == self.page.initial_items.len() - 1 {
                self.cur_idx = 0;
                self.use_initial = false;
            } else {
                self.cur_idx += 1;
            }
            Some(self.page.initial_items[idx].clone())
        } else {
            if self.need_next_page() {
                if self.fill_next_page().is_none() {
                    return None;
                }
            }
            let idx = self.cur_idx;
            self.cur_idx += 1;
            Some(self.buffer[idx].clone())
        }
    }
}
