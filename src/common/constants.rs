pub const GRAPHQL_ENDPOINT: &str = env!("GRAPHQL_URL");
pub const REST_ENDPOINT: &str = env!("REST_URL");
pub const BERKELEY_FEATURES_ENABLED: &str = env!("BERKELEY_FEATURES_ENABLED");
pub const TABLE_RECORD_SIZE: i64 = 100;
pub const TABLE_DEFAULT_PAGE_SIZE: usize = 10;
pub const ESTIMATED_ROW_HEIGHT: usize = 48;
pub const DEFAULT_ESTIMATED_NON_TABLE_SPACE_IN_SECTIONS: usize = 160;
pub const EPOCH_SLOTS: u16 = 7140;
pub const DEFAULT_USER_INPUT_DEBOUNCE_INTERNVAL: f64 = 500.0;
pub const GLOBAL_SEARCH_PLACEHOLDER_TEXT: &str =
    "Paste -> Enter -> Explore!";
pub const LIVE_RELOAD_INTERVAL: u64 = 60000;
