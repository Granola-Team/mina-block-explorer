use crate::internal_commands::graphql::internal_commands_query::{
    BlockQueryInput, InternalCommandQueryInput,
};
use chrono::Utc;
use graphql_client::GraphQLQuery;

type DateTime = chrono::DateTime<Utc>;
type Long = i32;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schemas/mina-explorer.graphql",
    query_path = "./graphql/queries/internal_commands.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone,Default",
    skip_serializing_none
)]
pub struct InternalCommandsQuery;

#[allow(clippy::derivable_impls)]
impl Default for InternalCommandQueryInput {
    fn default() -> Self {
        InternalCommandQueryInput {
            block_height: None,
            block_state_hash: None,
            canonical: None,
            recipient: None,
            block_height_gt: None,
            block_height_gte: None,
            block_height_lt: None,
            block_height_lte: None,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for BlockQueryInput {
    fn default() -> Self {
        BlockQueryInput {
            creator_nin: None,
            state_hash_lte: None,
            canonical_ne: None,
            date_time_lt: None,
            snark_jobs: None,
            creator_ne: None,
            received_time: None,
            block_height_lte: None,
            state_hash_field_gte: None,
            received_time_in: None,
            block_height: None,
            state_hash_field_exists: None,
            block_height_nin: None,
            creator_gt: None,
            state_hash_gte: None,
            state_hash_lt: None,
            creator_gte: None,
            protocol_state: None,
            canonical_exists: None,
            date_time_nin: None,
            creator_lte: None,
            creator_account: None,
            state_hash_field_lt: None,
            creator_in: None,
            state_hash_ne: None,
            received_time_ne: None,
            creator: None,
            state_hash_field_lte: None,
            date_time_lte: None,
            date_time_exists: None,
            state_hash_field_gt: None,
            date_time: None,
            date_time_gt: None,
            winner_account_exists: None,
            received_time_gte: None,
            protocol_state_exists: None,
            state_hash_exists: None,
            canonical: None,
            creator_exists: None,
            received_time_lte: None,
            block_height_exists: None,
            state_hash_field_ne: None,
            winner_account: None,
            or: None,
            state_hash_field: None,
            received_time_lt: None,
            transactions: None,
            date_time_gte: None,
            and: None,
            creator_account_exists: None,
            block_height_in: None,
            received_time_nin: None,
            snark_jobs_nin: None,
            date_time_in: None,
            snark_jobs_in: None,
            block_height_lt: None,
            state_hash_field_in: None,
            block_height_ne: None,
            transactions_exists: None,
            creator_lt: None,
            received_time_exists: None,
            block_height_gt: None,
            state_hash_nin: None,
            state_hash_field_nin: None,
            date_time_ne: None,
            state_hash_in: None,
            state_hash: None,
            block_height_gte: None,
            received_time_gt: None,
            snark_jobs_exists: None,
            state_hash_gt: None,
        }
    }
}
