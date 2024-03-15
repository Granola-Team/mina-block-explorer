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

pub struct Pagination {
    pub current_page: usize,
    pub records_per_page: usize,
    pub total_records: usize,
    pub set_current_page: WriteSignal<usize>,
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

pub enum ColorVariant {
    Green,
    Blue,
    Orange,
    Grey,
    Transparent,
    DarkBlue,
    Purple
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
            set_current_page: set_page,
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
            set_current_page: set_page,
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
pub struct BooleanUrlParamSelectOptions {
    pub true_case: String,
    pub false_case: String,
}

#[derive(Clone, PartialEq)]
pub enum NavIcon {
    Blocks,
    Transactions,
    Accounts,
    SNARKs,
    Staking,
    More,
    Broadcast,
    ZKApps,
    Tokens,
    Addresses,
    FeeTransfers,
    Analytics,
}
