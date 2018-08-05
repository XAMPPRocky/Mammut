use page::Page;
use entities::{
    account::Account,
    notification::Notification,
    relationship::Relationship,
    report::Report,
    status::{Emoji, Status}
};
use serde::Deserialize;

macro_rules! into_pageiter {
    ($typ:ty) => {
        impl<'a> IntoIterator for Page<'a, $typ> {
            type Item = $typ;
            type IntoIter = PageIter<'a, $typ>;

            fn into_iter(self) -> PageIter<'a, $typ> {
                PageIter::new(self)
            }
        }
    }
}

into_pageiter!(Status);
into_pageiter!(Account);
into_pageiter!(String);
into_pageiter!(Emoji);
into_pageiter!(Notification);
into_pageiter!(Report);
into_pageiter!(Relationship);

pub struct PageIter<'a, T: Clone + for<'de> Deserialize<'de>> {
    page: Page<'a, T>,
    buffer: Vec<T>,
    cur_idx: usize,
    use_initial: bool,
}

impl<'a, T: Clone + for<'de> Deserialize<'de>> PageIter<'a, T> {
    fn new(page: Page<'a, T>) -> PageIter<'a, T> {
        PageIter {
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

impl<'a, T: Clone+ for<'de> Deserialize<'de>> Iterator for PageIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.use_initial {
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
