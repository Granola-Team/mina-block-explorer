use graphql_client::Error;
use leptos::{web_sys::MouseEvent, *};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MyError {
    NetworkError(String),
    ParseError(String), // other error variants
    GraphQLError(Vec<Error>),
    GraphQLEmpty(String),
}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> Self {
        MyError::NetworkError(err.to_string())
    }
}

pub enum Status {
    Pending,
    Complete,
    Unknown,
}

#[derive(Clone)]
pub struct Pagination {
    pub current_page: usize,
    pub records_per_page: usize,
    pub total_records: usize,
    pub next_page: Callback<MouseEvent>,
    pub prev_page: Callback<MouseEvent>,
}

impl Pagination {
    pub fn start_index(&self) -> usize {
        self.current_page * self.records_per_page - self.records_per_page + 1
    }

    pub fn end_index(&self) -> usize {
        self.current_page * self.records_per_page
    }

    pub fn total_pages(&self) -> usize {
        self.total_records / self.records_per_page
            + (self.total_records % self.records_per_page).clamp(0, 1)
    }
}

pub enum PillVariant {
    Green,
    Blue,
    Orange,
}

#[cfg(test)]
mod pagination_tests {
    use super::*;

    #[test]
    fn test_indexes_first_page() {
        let (_, set_page) = create_signal(1);
        let pd = Pagination {
            current_page: 1,
            records_per_page: 15,
            total_records: 90,
            next_page: Callback::from(move |_| {
                let set_current_page_inner = set_page;
                set_current_page_inner.update(|cp| *cp += 1);
            }),
            prev_page: Callback::from(move |_| {
                let set_current_page_inner = set_page;
                set_current_page_inner.update(|cp| *cp -= 1);
            }),
        };
        assert_eq!(pd.start_index(), 1);
        assert_eq!(pd.end_index(), 15)
    }

    #[test]
    fn test_indexes_second_page() {
        let (_, set_page) = create_signal(1);
        let pd = Pagination {
            current_page: 2,
            records_per_page: 15,
            total_records: 90,
            next_page: Callback::from(move |_| {
                let set_current_page_inner = set_page;
                set_current_page_inner.update(|cp| *cp += 1);
            }),
            prev_page: Callback::from(move |_| {
                let set_current_page_inner = set_page;
                set_current_page_inner.update(|cp| *cp -= 1);
            }),
        };
        assert_eq!(pd.start_index(), 16);
        assert_eq!(pd.end_index(), 30)
    }

    #[test]
    fn test_total_pages() {
        let (_, set_page) = create_signal(1);
        let pd = Pagination {
            current_page: 2,
            records_per_page: 15,
            total_records: 90,
            next_page: Callback::from(move |_| {
                let set_current_page_inner = set_page;
                set_current_page_inner.update(|cp| *cp += 1);
            }),
            prev_page: Callback::from(move |_| {
                let set_current_page_inner = set_page;
                set_current_page_inner.update(|cp| *cp -= 1);
            }),
        };
        assert_eq!(pd.total_pages(), 6);
        let pd = Pagination {
            current_page: 2,
            records_per_page: 15,
            total_records: 91,
            next_page: Callback::from(move |_| {
                let set_current_page_inner = set_page;
                set_current_page_inner.update(|cp| *cp += 1);
            }),
            prev_page: Callback::from(move |_| {
                let set_current_page_inner = set_page;
                set_current_page_inner.update(|cp| *cp -= 1);
            }),
        };
        assert_eq!(pd.total_pages(), 7);
    }
}
