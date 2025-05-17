use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PublicKey(String);

impl PublicKey {
    pub const LEN: usize = 55;
    pub const PREFIX: &'static str = "B62q";

    pub fn new(pk: impl Into<String>) -> Result<Self, String> {
        let pk = pk.into();
        if Self::is_valid(&pk) {
            Ok(Self(pk))
        } else {
            Err(format!("Invalid public key: {}", pk))
        }
    }

    fn is_valid(pk: &str) -> bool {
        pk.starts_with(Self::PREFIX) && pk.len() == Self::LEN
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for PublicKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for PublicKey {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<String> for PublicKey {
    fn from(val: String) -> Self {
        let expectation = format!(
            "String must be a valid public key ({} chars, starts with '{}')",
            PublicKey::LEN,
            PublicKey::PREFIX
        );
        PublicKey::new(val).expect(&expectation)
    }
}

impl From<&str> for PublicKey {
    fn from(val: &str) -> Self {
        let expectation = format!(
            "String must be a valid public key ({} chars, starts with '{}')",
            PublicKey::LEN,
            PublicKey::PREFIX
        );
        PublicKey::new(val).expect(&expectation)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TransactionKind {
    Payment,
    Zkapp,
    StakeDelegation,
}

impl fmt::Display for TransactionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TransactionKind::Payment => "PAYMENT",
            TransactionKind::Zkapp => "ZKAPP",
            TransactionKind::StakeDelegation => "STAKE_DELEGATION",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for TransactionKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PAYMENT" => Ok(TransactionKind::Payment),
            "ZKAPP" => Ok(TransactionKind::Zkapp),
            "STAKE_DELEGATION" => Ok(TransactionKind::StakeDelegation),
            _ => Err(format!("Invalid TransactionKind: {}", s)),
        }
    }
}

pub enum ButtonStyleVariant {
    Primary,
    Secondary,
    Tertiary,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TableMetadata {
    pub total_records: Option<u64>,
    pub displayed_records: u64,
    pub available_records: Option<u64>,
}

pub struct TableMetadataBuilder {
    total_records_options: Vec<(Box<dyn Fn() -> bool>, u64)>,
    displayed_records_options: Vec<(Box<dyn Fn() -> bool>, u64)>,
    available_records_options: Vec<(Box<dyn Fn() -> bool>, u64)>,
}

impl TableMetadataBuilder {
    pub fn new() -> Self {
        Self {
            total_records_options: Vec::new(),
            displayed_records_options: Vec::new(),
            available_records_options: Vec::new(),
        }
    }

    // Add an option for total_records
    #[allow(dead_code)]
    pub fn total_records(mut self, condition: impl Fn() -> bool + 'static, value: u64) -> Self {
        self.total_records_options
            .push((Box::new(condition), value));
        self
    }

    // Add an option for displayed_records
    #[allow(dead_code)]
    pub fn displayed_records(mut self, condition: impl Fn() -> bool + 'static, value: u64) -> Self {
        self.displayed_records_options
            .push((Box::new(condition), value));
        self
    }

    // Add an option for available_records
    pub fn available_records(mut self, condition: impl Fn() -> bool + 'static, value: u64) -> Self {
        self.available_records_options
            .push((Box::new(condition), value));
        self
    }

    // Unconditional total_records
    pub fn total_records_value(mut self, value: u64) -> Self {
        self.total_records_options
            .push((Box::new(move || true), value));
        self
    }

    // Unconditional displayed_records
    pub fn displayed_records_value(mut self, value: u64) -> Self {
        self.displayed_records_options
            .push((Box::new(move || true), value));
        self
    }

    // Unconditional available_records
    #[allow(dead_code)]
    pub fn available_records_value(mut self, value: u64) -> Self {
        self.available_records_options
            .push((Box::new(move || true), value));
        self
    }

    pub fn build(self) -> TableMetadata {
        // Helper to select the first matching option or a default
        let select_option = |options: Vec<(Box<dyn Fn() -> bool>, u64)>| -> u64 {
            for (condition, value) in options {
                if condition() {
                    return value;
                }
            }
            0 // Default value
        };

        TableMetadata {
            total_records: {
                let value = select_option(self.total_records_options);
                if value == 0 { None } else { Some(value) }
            },
            displayed_records: select_option(self.displayed_records_options),
            available_records: {
                let value = select_option(self.available_records_options);
                if value == 0 { None } else { Some(value) }
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MyError {
    NetworkError(String),
    ParseError(String), // other error variants
    GraphQLError(Vec<graphql_client::Error>),
    GraphQLEmpty(String),
    UrlParseError(String),
}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> Self {
        MyError::NetworkError(err.to_string())
    }
}

pub enum ColorVariant {
    Green,
    Blue,
    Grey,
    Orange,
    DarkGreen,
    DarkBlue,
    DarkGrey,
}

#[derive(Clone)]
pub enum NavMatchType {
    Exact,
    Prefix,
}

#[derive(Clone)]
pub struct NavEntry {
    pub href: String,
    pub text: String,
    pub icon: NavIcon,
    pub sub_entries: Option<Vec<NavEntry>>,
    pub disabled: bool,
    pub number_bubble: Option<usize>,
    pub match_type: Option<NavMatchType>,
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
            match_type: Some(NavMatchType::Exact),
        }
    }
}

impl NavEntry {
    pub fn is_match(&self, pathname: &str) -> bool {
        self.match_type.as_ref().is_some_and(|mt| match mt {
            NavMatchType::Exact => self.href == pathname,
            NavMatchType::Prefix => pathname.starts_with(&self.href),
        })
    }
}

#[cfg(test)]
mod nav_entry_tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let nav_entry = NavEntry {
            href: "/home".to_string(),
            match_type: Some(NavMatchType::Exact),
            ..Default::default()
        };
        assert!(nav_entry.is_match("/home"));
        assert!(!nav_entry.is_match("/home/about"));
    }

    #[test]
    fn test_prefix_match() {
        let nav_entry = NavEntry {
            href: "/home".to_string(),
            match_type: Some(NavMatchType::Prefix),
            ..Default::default()
        };
        assert!(nav_entry.is_match("/home"));
        assert!(nav_entry.is_match("/home/about"));
        assert!(!nav_entry.is_match("/about/home"));
    }

    #[test]
    fn test_no_match_type() {
        let nav_entry = NavEntry {
            href: "/home".to_string(),
            match_type: None,
            ..Default::default()
        };
        assert!(!nav_entry.is_match("/home"));
        assert!(!nav_entry.is_match("/home/about"));
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
    #[allow(dead_code)]
    ZKApps,
    Tokens,
    Addresses,
    FeeTransfers,
    Analytics,
    More,
    Delegates,
    Leaderboard,
}

pub trait HasBlockHeight {
    fn block_height(&self) -> Option<i64>;
}

#[cfg(test)]
mod table_metadata_builder_tests {
    use super::*;
    use std::{cell::Cell, rc::Rc};

    // Helper to create a builder with no options
    fn empty_builder() -> TableMetadataBuilder {
        TableMetadataBuilder::new()
    }

    #[test]
    fn test_empty_builder() {
        let metadata = empty_builder().build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: None,
                displayed_records: 0,
                available_records: None,
            },
            "Empty builder should return default metadata"
        );
    }

    #[test]
    fn test_unconditional_values() {
        let metadata = empty_builder()
            .total_records_value(1000)
            .displayed_records_value(50)
            .available_records_value(200)
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: Some(1000),
                displayed_records: 50,
                available_records: Some(200),
            },
            "Unconditional values should set fields directly"
        );
    }

    #[test]
    fn test_conditional_total_records() {
        let condition1 = true;
        let condition2 = false;
        let metadata = empty_builder()
            .total_records(move || condition1, 1000)
            .total_records(move || condition2, 2000)
            .build();
        assert_eq!(
            metadata.total_records,
            Some(1000),
            "First true condition should set total_records"
        );

        let condition1 = false;
        let condition2 = true;
        let metadata = empty_builder()
            .total_records(move || condition1, 1000)
            .total_records(move || condition2, 2000)
            .build();
        assert_eq!(
            metadata.total_records,
            Some(2000),
            "Second true condition should set total_records"
        );
    }

    #[test]
    fn test_conditional_displayed_records() {
        let condition1 = true;
        let condition2 = false;
        let metadata = empty_builder()
            .displayed_records(move || condition1, 50)
            .displayed_records(move || condition2, 100)
            .build();
        assert_eq!(
            metadata.displayed_records, 50,
            "First true condition should set displayed_records"
        );

        let condition1 = false;
        let condition2 = true;
        let metadata = empty_builder()
            .displayed_records(move || condition1, 50)
            .displayed_records(move || condition2, 100)
            .build();
        assert_eq!(
            metadata.displayed_records, 100,
            "Second true condition should set displayed_records"
        );
    }

    #[test]
    fn test_conditional_available_records() {
        let condition1 = true;
        let condition2 = false;
        let metadata = empty_builder()
            .available_records(move || condition1, 200)
            .available_records(move || condition2, 400)
            .build();
        assert_eq!(
            metadata.available_records,
            Some(200),
            "First true condition should set available_records"
        );

        let condition1 = false;
        let condition2 = true;
        let metadata = empty_builder()
            .available_records(move || condition1, 200)
            .available_records(move || condition2, 400)
            .build();
        assert_eq!(
            metadata.available_records,
            Some(400),
            "Second true condition should set available_records"
        );
    }

    #[test]
    fn test_no_matching_conditions() {
        let metadata = empty_builder()
            .total_records(move || false, 1000)
            .displayed_records(move || false, 50)
            .available_records(move || false, 200)
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: None,
                displayed_records: 0,
                available_records: None,
            },
            "No matching conditions should return defaults"
        );
    }

    #[test]
    fn test_zero_values() {
        let metadata = empty_builder()
            .total_records_value(0)
            .displayed_records_value(0)
            .available_records_value(0)
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: None,
                displayed_records: 0,
                available_records: None,
            },
            "Zero values should map to None for Option fields"
        );
    }

    #[test]
    fn test_multiple_conditions_same_field() {
        let q_type: Option<String> = Some("zkapp".to_string());
        let q_type_clone = q_type.clone();
        let metadata = empty_builder()
            .available_records(move || q_type.is_none(), 1000)
            .available_records(
                move || q_type_clone.as_ref().map(|t| t == "zkapp").unwrap_or(false),
                200,
            )
            .available_records(move || true, 500) // Fallback
            .build();
        assert_eq!(
            metadata.available_records,
            Some(200),
            "Correct condition should be selected based on q_type"
        );

        {
            let q_type: Option<String> = None;
            let q_type_clone = q_type.clone();
            let metadata = empty_builder()
                .available_records(move || q_type.is_none(), 1000)
                .available_records(
                    move || q_type_clone.as_ref().map(|t| t == "zkapp").unwrap_or(false),
                    200,
                )
                .available_records(move || true, 500)
                .build();
            assert_eq!(
                metadata.available_records,
                Some(1000),
                "Correct condition should be selected when q_type is None"
            );
        }
    }

    #[test]
    fn test_conflicting_conditions() {
        let metadata = empty_builder()
            .total_records(move || true, 1000)
            .total_records(move || true, 2000) // Conflicting true condition
            .build();
        assert_eq!(
            metadata.total_records,
            Some(1000),
            "First true condition should take precedence"
        );
    }

    #[test]
    fn test_mixed_conditional_unconditional() {
        let q_type: Option<String> = Some("zkapp".to_string());
        let q_type_clone = q_type.clone();
        let metadata = empty_builder()
            .total_records_value(1000) // Unconditional
            .displayed_records(move || false, 25)
            .displayed_records_value(50) // Unconditional
            .available_records(
                move || q_type.as_ref().map(|t| t == "zkapp").unwrap_or(false),
                200,
            )
            .available_records(move || q_type_clone.is_none(), 1000)
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: Some(1000),
                displayed_records: 50,
                available_records: Some(200),
            },
            "Mix of conditional and unconditional options should work"
        );
    }

    #[test]
    fn test_chainability() {
        let metadata = TableMetadataBuilder::new()
            .total_records_value(1000)
            .total_records(move || false, 2000)
            .displayed_records_value(50)
            .displayed_records(move || false, 100)
            .available_records_value(200)
            .available_records(move || false, 400)
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: Some(1000),
                displayed_records: 50,
                available_records: Some(200),
            },
            "Chaining multiple calls should work correctly"
        );
    }

    #[test]
    fn test_large_values() {
        let metadata = empty_builder()
            .total_records_value(u64::MAX)
            .displayed_records_value(u64::MAX)
            .available_records_value(u64::MAX)
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: Some(u64::MAX),
                displayed_records: u64::MAX,
                available_records: Some(u64::MAX),
            },
            "Large values should be handled correctly"
        );
    }

    #[test]
    fn test_no_options_for_some_fields() {
        let metadata = empty_builder().total_records_value(1000).build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: Some(1000),
                displayed_records: 0,
                available_records: None,
            },
            "Fields with no options should use defaults"
        );
    }

    #[test]
    fn test_condition_side_effects() {
        let counter = Rc::new(Cell::new(0));
        let counter_clone = Rc::clone(&counter); // Clone for the closure
        let condition = move || {
            counter_clone.set(counter_clone.get() + 1);
            true
        };
        let metadata = empty_builder().total_records(condition, 1000).build();
        assert_eq!(
            metadata.total_records,
            Some(1000),
            "Condition should be evaluated correctly"
        );
        assert_eq!(counter.get(), 1, "Condition should be called exactly once");
    }

    #[test]
    fn test_multiple_builds() {
        let builder = empty_builder()
            .total_records_value(1000)
            .displayed_records_value(50)
            .available_records_value(200);
        let metadata1 = builder.build();
        let metadata2 = empty_builder().build(); // Simulate reusing builder pattern
        assert_eq!(
            metadata1,
            TableMetadata {
                total_records: Some(1000),
                displayed_records: 50,
                available_records: Some(200),
            },
            "First build should have correct values"
        );
        assert_eq!(
            metadata2,
            TableMetadata {
                total_records: None,
                displayed_records: 0,
                available_records: None,
            },
            "New builder should start fresh"
        );
    }
}
