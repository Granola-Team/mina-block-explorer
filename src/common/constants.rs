pub const GRAPHQL_ENDPOINT: &str = env!("GRAPHQL_URL");
pub const REST_ENDPOINT: &str = env!("REST_URL");
pub const MINA_EXPLORER_ENDPOINT: &str = "https://api.minaexplorer.com";
pub const EPOCH_SLOTS: u16 = 7140;
pub const DEFAULT_USER_INPUT_DEBOUNCE_INTERNVAL: f64 = 500.0;
pub const GLOBAL_SEARCH_PLACEHOLDER_TEXT: &str = "Paste -> Enter -> Explore!";
pub const LIVE_RELOAD_INTERVAL: u64 = 60000;
pub const TABLE_ROW_LIMIT: u64 = 25;
pub const COMMIT_HASH: &str = env!("COMMIT_HASH");
pub const LINK_HOVER_STATE: &str = "hover:text-granola-orange hover:underline hover:decoration-2 ";
pub const TABLE_COL_NUMERIC_WIDTH: &str = "150px";
pub const TABLE_COL_SHORT_WIDTH: &str = "150px";
pub const TABLE_COL_DATE_WIDTH: &str = "280px";
pub const TABLE_COL_HASH_WIDTH: &str = "470px";
pub const TABLE_COL_USERNAME_WIDTH: &str = "200rem";
pub const TABLE_COL_LARGE_BALANCE: &str = "250px";
pub const BLOCKCHAIN_SUMMARY_STORAGE_KEY: &str = "blockchain-summary";
pub const BLOCKS_STORAGE_KEY: &str = "blocks";
pub const PAYMENT_TYPE: &str = "Payment";
pub const STAKE_DELEGATION_TYPE: &str = "Stake Delegation";
pub const LHS_MAX_DIGIT_PADDING: usize = 3;
pub const LHS_MAX_SPACE_FEES: usize = 2;
pub const TXN_STATUS_APPLIED: &str = "Applied";
pub const TXN_STATUS_FAILED: &str = "Failed";
pub const DEFAULT_INPUT_STYLES: &str = "block h-6 text-base text-sm font-normal font-mono p-2 text-right border rounded-sm border-slate-400 focus:border-granola-orange";
pub const QUERY_PARAM_BLOCKHEIGHT_GTE: &str = "q-blockheight-gte";
pub const QUERY_PARAM_BLOCKHEIGHT_LTE: &str = "q-blockheight-lte";
