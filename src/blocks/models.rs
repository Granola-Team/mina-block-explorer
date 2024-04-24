use super::graphql::blocks_query::BlocksQueryBlocks;

pub struct SummaryPageBlocksQueryBlocks(pub Vec<Option<BlocksQueryBlocks>>);

#[derive(Clone)]
pub enum BlockContent {
    Spotlight,
    UserCommands,
    FeeTransfers,
    SNARKJobs,
    Analytics,
}

pub struct BlockMultiSearch {
    pub block_height: Option<i64>,
    pub public_key: Option<String>,
    pub state_hash: Option<String>,
}
