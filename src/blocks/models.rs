use super::graphql::blocks_query::BlocksQueryBlocks;

pub struct SummaryPageBlocksQueryBlocks(pub Vec<Option<BlocksQueryBlocks>>);

#[derive(Clone)]
pub enum BlockContent {
    Spotlight,
    UserCommands,
    FeeTransfers,
    SNARKJobs,
    ZKApps,
}
