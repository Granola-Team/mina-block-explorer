pub const GRAPHQL_ENDPOINT: &str = env!("GRAPHQL_URL");
pub const REST_ENDPOINT: &str = env!("REST_URL");
pub const MINA_EXPLORER_ENDPOINT: &str = "https://api.minaexplorer.com";
pub const EPOCH_SLOTS: u16 = 7140;
pub const DEFAULT_USER_INPUT_DEBOUNCE_INTERNVAL: f64 = 500.0;
pub const GLOBAL_SEARCH_PLACEHOLDER_TEXT: &str = "Paste -> Enter -> Explore!";
pub const LIVE_RELOAD_INTERVAL: u64 = 6000000;
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
pub const STAKE_DELEGATION_TYPE: &str = "Stake Delegation";
pub const LHS_MAX_DIGIT_PADDING: usize = 3;
pub const LHS_MAX_SPACE_FEES: usize = 2;
pub const TXN_STATUS_APPLIED: &str = "Applied";
pub const TXN_STATUS_FAILED: &str = "Failed";
pub const DEFAULT_INPUT_STYLES: &str = "block h-6 text-base text-sm font-normal font-mono p-2 text-right border rounded-sm border-slate-400 focus:border-granola-orange";
pub const QUERY_PARAM_BLOCKHEIGHT_GTE: &str = "q-blockheight-gte";
pub const QUERY_PARAM_BLOCKHEIGHT_LTE: &str = "q-blockheight-lte";
pub const QUERY_PARAM_RECEIPIENT: &str = "q-recipient";
pub const QUERY_PARAM_HEIGHT: &str = "q-height";
pub const QUERY_PARAM_ROW_LIMIT: &str = "row-limit";
pub const QUERY_PARAM_BALANCE: &str = "q-balance";
pub const QUERY_PARAM_TOKEN: &str = "q-token";
pub const QUERY_PARAM_STAKE: &str = "q-stake";
pub const QUERY_PARAM_DELEGATE: &str = "q-delegate";
pub const QUERY_PARAM_TOKEN_SYMBOL: &str = "q-symbol";
pub const QUERY_PARAM_TYPE: &str = "q-type";
pub const QUERY_PARAM_TXN_APPLIED: &str = "q-status";
pub const QUERY_PARAM_TO: &str = "q-to";
pub const TYPE_SEARCH_OPTION_ZKAPP: &str = "Zkapp";
pub const MINA_TOKEN_ADDRESS: &str = "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf";
pub const MAINNET_STATE_HASH: &str = "3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ";
pub const HARDFORK_STATE_HASH: &str = "3NK4BpDSekaqsG6tx8Nse2zJchRft2JpnbvMiog55WCr5xJZaKeP";
pub const BERKELEY_CHAIN_ID: &str = "a7351a";
pub const MAINNET_CHAIN_ID: &str = "5f704c";
pub const LAST_EPOCH_OF_MAINNET_CHAIN: u64 = 80;
