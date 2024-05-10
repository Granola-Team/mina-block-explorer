use graphql_client::Error;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MyError {
    NetworkError(String),
    ParseError(String), // other error variants
    GraphQLError(Vec<Error>),
    GraphQLEmpty(String),
    UrlParseError(String),
}

impl From<url::ParseError> for MyError {
    fn from(err: url::ParseError) -> MyError {
        MyError::UrlParseError(err.to_string())
    }
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
    pub set_current_page: WriteSignal<usize>,
}
impl Pagination {
    pub fn start_index(&self) -> usize {
        if self.total_records == 0 {
            0
        } else {
            (self.current_page - 1) * self.records_per_page + 1
        }
    }

    pub fn end_index(&self) -> usize {
        if self.total_records == 0 {
            0
        } else {
            std::cmp::min(self.current_page * self.records_per_page, self.total_records)
        }
    }

    pub fn total_pages(&self) -> usize {
        if self.total_records == 0 {
            0
        } else {
            (self.total_records + self.records_per_page - 1) / self.records_per_page
        }
    }
}

#[cfg(test)]
mod pagination_tests {
    use super::*;

    #[test]
    fn test_indexes_zero_records() {
        let (_, set_page) = create_signal(1);
        let pd = Pagination {
            current_page: 1,
            records_per_page: 15,
            total_records: 0,
            set_current_page: set_page,
        };
        assert_eq!(pd.start_index(), 0);
        assert_eq!(pd.end_index(), 0);
    }

    #[test]
    fn test_indexes_first_page() {
        let (_, set_page) = create_signal(1);
        let pd = Pagination {
            current_page: 1,
            records_per_page: 15,
            total_records: 90,
            set_current_page: set_page,
        };
        assert_eq!(pd.start_index(), 1);
        assert_eq!(pd.end_index(), 15);
    }

    #[test]
    fn test_indexes_second_page() {
        let (_, set_page) = create_signal(1);
        let pd = Pagination {
            current_page: 2,
            records_per_page: 15,
            total_records: 90,
            set_current_page: set_page,
        };
        assert_eq!(pd.start_index(), 16);
        assert_eq!(pd.end_index(), 30);
    }

    #[test]
    fn test_total_pages() {
        let (_, set_page) = create_signal(1);
        let pd = Pagination {
            current_page: 2,
            records_per_page: 15,
            total_records: 90,
            set_current_page: set_page,
        };
        assert_eq!(pd.total_pages(), 6);
        let pd = Pagination {
            current_page: 2,
            records_per_page: 15,
            total_records: 91,
            set_current_page: set_page,
        };
        assert_eq!(pd.total_pages(), 7);
    }

    // Test for total records less than records per page
    #[test]
    fn test_fewer_records_than_page_size() {
        let (_, set_page) = create_signal(1);
        let pd = Pagination {
            current_page: 1,
            records_per_page: 15,
            total_records: 10,
            set_current_page: set_page,
        };
        assert_eq!(pd.start_index(), 1);
        assert_eq!(pd.end_index(), 10);
        assert_eq!(pd.total_pages(), 1);
    }
}

pub enum ColorVariant {
    Green,
    Blue,
    Grey,
    Transparent,
    DarkBlue,
}

#[derive(Clone)]
pub struct NavEntry {
    pub href: String,
    pub text: String,
    pub icon: NavIcon,
    pub sub_entries: Option<Vec<NavEntry>>,
    pub disabled: bool,
    pub number_bubble: Option<usize>,
}

impl Default for NavEntry {
    fn default() -> Self {
        NavEntry {
            sub_entries: None,
            disabled: false,
            number_bubble: None,
            href: String::new(),
            text: String::new(),
            icon: NavIcon::Accounts,
        }
    }
}

#[derive(Clone)]
pub struct UrlParamSelectOptions {
    pub is_boolean_option: bool,
    pub cases: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub enum NavIcon {
    Blocks,
    Transactions,
    Accounts,
    SNARKs,
    Staking,
    Send,
    ZKApps,
    Tokens,
    Addresses,
    FeeTransfers,
    Analytics,
}

#[derive(Clone, Debug)]
pub struct PageDimensions {
    pub height: Option<f64>,
    pub width: Option<f64>,
}
