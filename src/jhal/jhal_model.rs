use crate::entities::{authors, books, series};

#[cfg(feature = "search")]
use crate::entities::search_index;

pub trait JhalModel {
    fn resource_name(&self) -> &str;

    fn list_href(&self) -> String {
        format!("/{}", self.resource_name())
    }

    fn item_href(&self) -> String {
        format!("{}/{{}}", self.list_href())
    }

    fn self_href(&self, id: i32) -> String {
        format!("{}/{id}", self.list_href())
    }
}

impl JhalModel for authors::Model {
    fn resource_name(&self) -> &str {
        "authors"
    }
}

impl JhalModel for books::Model {
    fn resource_name(&self) -> &str {
        "books"
    }
}

impl JhalModel for series::Model {
    fn resource_name(&self) -> &str {
        "series"
    }
}

#[cfg(feature = "search")]
impl JhalModel for search_index::Model {
    fn resource_name(&self) -> &str {
        "search"
    }
}
