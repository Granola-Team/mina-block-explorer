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

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TableMetadata {
    pub total_records: Option<u64>,
    pub displayed_records: u64,
    pub available_records: Option<u64>,
    pub total_records_hint: Option<String>,
    pub displayed_records_hint: Option<String>,
    pub available_records_hint: Option<String>,
}

type ConditionalTableMetadataRecords = Vec<(Box<dyn Fn() -> bool>, u64, Option<String>)>;

pub struct TableMetadataBuilder {
    total_records_options: ConditionalTableMetadataRecords,
    displayed_records_options: ConditionalTableMetadataRecords,
    available_records_options: ConditionalTableMetadataRecords,
}

impl TableMetadataBuilder {
    pub fn new() -> Self {
        Self {
            total_records_options: Vec::new(),
            displayed_records_options: Vec::new(),
            available_records_options: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn total_records(
        mut self,
        condition: impl Fn() -> bool + 'static,
        value: u64,
        hint: Option<String>,
    ) -> Self {
        self.total_records_options
            .push((Box::new(condition), value, hint));
        self
    }

    #[allow(dead_code)]
    pub fn displayed_records(
        mut self,
        condition: impl Fn() -> bool + 'static,
        value: u64,
        hint: Option<String>,
    ) -> Self {
        self.displayed_records_options
            .push((Box::new(condition), value, hint));
        self
    }

    pub fn available_records(
        mut self,
        condition: impl Fn() -> bool + 'static,
        value: u64,
        hint: Option<String>,
    ) -> Self {
        self.available_records_options
            .push((Box::new(condition), value, hint));
        self
    }

    pub fn total_records_value(mut self, value: u64, hint: Option<String>) -> Self {
        self.total_records_options
            .push((Box::new(|| true), value, hint));
        self
    }

    pub fn displayed_records_value(mut self, value: u64, hint: Option<String>) -> Self {
        self.displayed_records_options
            .push((Box::new(|| true), value, hint));
        self
    }

    #[allow(dead_code)]
    pub fn available_records_value(mut self, value: u64, hint: Option<String>) -> Self {
        self.available_records_options
            .push((Box::new(|| true), value, hint));
        self
    }

    pub fn build(self) -> TableMetadata {
        let select_option = |options: ConditionalTableMetadataRecords| {
            for (condition, value, hint) in options {
                if condition() {
                    return (value, hint);
                }
            }
            (0, None) // Default value and hint
        };

        let (total_records_value, total_records_hint) = select_option(self.total_records_options);
        let (displayed_records_value, displayed_records_hint) =
            select_option(self.displayed_records_options);
        let (available_records_value, available_records_hint) =
            select_option(self.available_records_options);

        TableMetadata {
            total_records: if total_records_value == 0 {
                None
            } else {
                Some(total_records_value)
            },
            displayed_records: displayed_records_value,
            available_records: if available_records_value == 0 {
                None
            } else {
                Some(available_records_value)
            },
            total_records_hint,
            displayed_records_hint,
            available_records_hint,
        }
    }
}

#[cfg(test)]
mod table_metadata_builder_tests {
    use super::*;

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
                total_records_hint: None,
                displayed_records_hint: None,
                available_records_hint: None,
            },
            "Empty builder should return default metadata"
        );
    }

    #[test]
    fn test_unconditional_values() {
        let metadata = empty_builder()
            .total_records_value(1000, Some("Total rows in database".to_string()))
            .displayed_records_value(50, Some("Rows shown on page".to_string()))
            .available_records_value(200, Some("Rows accessible".to_string()))
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: Some(1000),
                displayed_records: 50,
                available_records: Some(200),
                total_records_hint: Some("Total rows in database".to_string()),
                displayed_records_hint: Some("Rows shown on page".to_string()),
                available_records_hint: Some("Rows accessible".to_string()),
            },
            "Unconditional values with hints should set fields directly"
        );
    }

    #[test]
    fn test_conditional_total_records() {
        let condition1 = true;
        let condition2 = false;
        let metadata = empty_builder()
            .total_records(
                move || condition1,
                1000,
                Some("Total rows when condition1".to_string()),
            )
            .total_records(
                move || condition2,
                2000,
                Some("Total rows when condition2".to_string()),
            )
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: Some(1000),
                displayed_records: 0,
                available_records: None,
                total_records_hint: Some("Total rows when condition1".to_string()),
                displayed_records_hint: None,
                available_records_hint: None,
            },
            "First true condition should set total_records with hint"
        );

        let condition1 = false;
        let condition2 = true;
        let metadata = empty_builder()
            .total_records(
                move || condition1,
                1000,
                Some("Total rows when condition1".to_string()),
            )
            .total_records(
                move || condition2,
                2000,
                Some("Total rows when condition2".to_string()),
            )
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: Some(2000),
                displayed_records: 0,
                available_records: None,
                total_records_hint: Some("Total rows when condition2".to_string()),
                displayed_records_hint: None,
                available_records_hint: None,
            },
            "Second true condition should set total_records with hint"
        );
    }

    #[test]
    fn test_conditional_displayed_records() {
        let condition1 = true;
        let condition2 = false;
        let metadata = empty_builder()
            .displayed_records(
                move || condition1,
                50,
                Some("Displayed rows when condition1".to_string()),
            )
            .displayed_records(
                move || condition2,
                100,
                Some("Displayed rows when condition2".to_string()),
            )
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: None,
                displayed_records: 50,
                available_records: None,
                total_records_hint: None,
                displayed_records_hint: Some("Displayed rows when condition1".to_string()),
                available_records_hint: None,
            },
            "First true condition should set displayed_records with hint"
        );

        let condition1 = false;
        let condition2 = true;
        let metadata = empty_builder()
            .displayed_records(
                move || condition1,
                50,
                Some("Displayed rows when condition1".to_string()),
            )
            .displayed_records(
                move || condition2,
                100,
                Some("Displayed rows when condition2".to_string()),
            )
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: None,
                displayed_records: 100,
                available_records: None,
                total_records_hint: None,
                displayed_records_hint: Some("Displayed rows when condition2".to_string()),
                available_records_hint: None,
            },
            "Second true condition should set displayed_records with hint"
        );
    }

    #[test]
    fn test_conditional_available_records() {
        let condition1 = true;
        let condition2 = false;
        let metadata = empty_builder()
            .available_records(
                move || condition1,
                200,
                Some("Available rows when condition1".to_string()),
            )
            .available_records(
                move || condition2,
                400,
                Some("Available rows when condition2".to_string()),
            )
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: None,
                displayed_records: 0,
                available_records: Some(200),
                total_records_hint: None,
                displayed_records_hint: None,
                available_records_hint: Some("Available rows when condition1".to_string()),
            },
            "First true condition should set available_records with hint"
        );

        let condition1 = false;
        let condition2 = true;
        let metadata = empty_builder()
            .available_records(
                move || condition1,
                200,
                Some("Available rows when condition1".to_string()),
            )
            .available_records(
                move || condition2,
                400,
                Some("Available rows when condition2".to_string()),
            )
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: None,
                displayed_records: 0,
                available_records: Some(400),
                total_records_hint: None,
                displayed_records_hint: None,
                available_records_hint: Some("Available rows when condition2".to_string()),
            },
            "Second true condition should set available_records with hint"
        );
    }

    #[test]
    fn test_no_matching_conditions() {
        let metadata = empty_builder()
            .total_records(
                move || false,
                1000,
                Some("Total rows when false".to_string()),
            )
            .displayed_records(
                move || false,
                50,
                Some("Displayed rows when false".to_string()),
            )
            .available_records(
                move || false,
                200,
                Some("Available rows when false".to_string()),
            )
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: None,
                displayed_records: 0,
                available_records: None,
                total_records_hint: None,
                displayed_records_hint: None,
                available_records_hint: None,
            },
            "No matching conditions should return defaults with no hints"
        );
    }

    #[test]
    fn test_zero_values() {
        let metadata = empty_builder()
            .total_records_value(0, Some("Zero total rows".to_string()))
            .displayed_records_value(0, Some("Zero displayed rows".to_string()))
            .available_records_value(0, Some("Zero available rows".to_string()))
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: None,
                displayed_records: 0,
                available_records: None,
                total_records_hint: Some("Zero total rows".to_string()),
                displayed_records_hint: Some("Zero displayed rows".to_string()),
                available_records_hint: Some("Zero available rows".to_string()),
            },
            "Zero values should map to None for Option fields, keeping hints"
        );
    }

    #[test]
    fn test_multiple_conditions_same_field() {
        let q_type: Option<String> = Some("zkapp".to_string());
        let q_type_clone = q_type.clone();
        let metadata = empty_builder()
            .available_records(
                move || q_type.is_none(),
                1000,
                Some("Available when no type".to_string()),
            )
            .available_records(
                move || q_type_clone.as_ref().map(|t| t == "zkapp").unwrap_or(false),
                200,
                Some("Available for zkapp".to_string()),
            )
            .available_records(move || true, 500, Some("Default available".to_string()))
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: None,
                displayed_records: 0,
                available_records: Some(200),
                total_records_hint: None,
                displayed_records_hint: None,
                available_records_hint: Some("Available for zkapp".to_string()),
            },
            "Correct condition should be selected with hint"
        );

        let q_type: Option<String> = None;
        let q_type_clone = q_type.clone();
        let metadata = empty_builder()
            .available_records(
                move || q_type.is_none(),
                1000,
                Some("Available when no type".to_string()),
            )
            .available_records(
                move || q_type_clone.as_ref().map(|t| t == "zkapp").unwrap_or(false),
                200,
                Some("Available for zkapp".to_string()),
            )
            .available_records(move || true, 500, Some("Default available".to_string()))
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: None,
                displayed_records: 0,
                available_records: Some(1000),
                total_records_hint: None,
                displayed_records_hint: None,
                available_records_hint: Some("Available when no type".to_string()),
            },
            "Correct condition should be selected when q_type is None with hint"
        );
    }

    #[test]
    fn test_conflicting_conditions() {
        let metadata = empty_builder()
            .total_records(move || true, 1000, Some("First total rows".to_string()))
            .total_records(move || true, 2000, Some("Second total rows".to_string()))
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: Some(1000),
                displayed_records: 0,
                available_records: None,
                total_records_hint: Some("First total rows".to_string()),
                displayed_records_hint: None,
                available_records_hint: None,
            },
            "First true condition should take precedence with its hint"
        );
    }

    #[test]
    fn test_mixed_conditional_unconditional() {
        let q_type: Option<String> = Some("zkapp".to_string());
        let q_type_clone = q_type.clone();
        let metadata = empty_builder()
            .total_records_value(1000, Some("Total rows".to_string()))
            .displayed_records(move || false, 25, Some("Displayed when false".to_string()))
            .displayed_records_value(50, Some("Displayed rows".to_string()))
            .available_records(
                move || q_type.as_ref().map(|t| t == "zkapp").unwrap_or(false),
                200,
                Some("Available for zkapp".to_string()),
            )
            .available_records(
                move || q_type_clone.is_none(),
                1000,
                Some("Available when no type".to_string()),
            )
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: Some(1000),
                displayed_records: 50,
                available_records: Some(200),
                total_records_hint: Some("Total rows".to_string()),
                displayed_records_hint: Some("Displayed rows".to_string()),
                available_records_hint: Some("Available for zkapp".to_string()),
            },
            "Mix of conditional and unconditional options should work with hints"
        );
    }

    #[test]
    fn test_chainability() {
        let metadata = TableMetadataBuilder::new()
            .total_records_value(1000, Some("Total rows".to_string()))
            .total_records(move || false, 2000, Some("Total when false".to_string()))
            .displayed_records_value(50, Some("Displayed rows".to_string()))
            .displayed_records(move || false, 100, Some("Displayed when false".to_string()))
            .available_records_value(200, Some("Available rows".to_string()))
            .available_records(move || false, 400, Some("Available when false".to_string()))
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: Some(1000),
                displayed_records: 50,
                available_records: Some(200),
                total_records_hint: Some("Total rows".to_string()),
                displayed_records_hint: Some("Displayed rows".to_string()),
                available_records_hint: Some("Available rows".to_string()),
            },
            "Chaining multiple calls should work correctly with hints"
        );
    }

    #[test]
    fn test_large_values() {
        let metadata = empty_builder()
            .total_records_value(u64::MAX, Some("Max total rows".to_string()))
            .displayed_records_value(u64::MAX, Some("Max displayed rows".to_string()))
            .available_records_value(u64::MAX, Some("Max available rows".to_string()))
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: Some(u64::MAX),
                displayed_records: u64::MAX,
                available_records: Some(u64::MAX),
                total_records_hint: Some("Max total rows".to_string()),
                displayed_records_hint: Some("Max displayed rows".to_string()),
                available_records_hint: Some("Max available rows".to_string()),
            },
            "Large values should be handled correctly with hints"
        );
    }

    #[test]
    fn test_no_options_for_some_fields() {
        let metadata = empty_builder()
            .total_records_value(1000, Some("Total rows".to_string()))
            .build();
        assert_eq!(
            metadata,
            TableMetadata {
                total_records: Some(1000),
                displayed_records: 0,
                available_records: None,
                total_records_hint: Some("Total rows".to_string()),
                displayed_records_hint: None,
                available_records_hint: None,
            },
            "Fields with no options should use defaults with no hints"
        );
    }

    #[test]
    fn test_multiple_builds() {
        let builder = empty_builder()
            .total_records_value(1000, Some("Total rows".to_string()))
            .displayed_records_value(50, Some("Displayed rows".to_string()))
            .available_records_value(200, Some("Available rows".to_string()));
        let metadata1 = builder.build();
        let metadata2 = empty_builder().build();
        assert_eq!(
            metadata1,
            TableMetadata {
                total_records: Some(1000),
                displayed_records: 50,
                available_records: Some(200),
                total_records_hint: Some("Total rows".to_string()),
                displayed_records_hint: Some("Displayed rows".to_string()),
                available_records_hint: Some("Available rows".to_string()),
            },
            "First build should have correct values with hints"
        );
        assert_eq!(
            metadata2,
            TableMetadata {
                total_records: None,
                displayed_records: 0,
                available_records: None,
                total_records_hint: None,
                displayed_records_hint: None,
                available_records_hint: None,
            },
            "New builder should start fresh with no hints"
        );
    }
}
