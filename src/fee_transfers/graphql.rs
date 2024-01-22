use chrono::Utc;
use graphql_client::GraphQLQuery;

use self::fee_transfers_query::FeetransferQueryInput;
type DateTime = chrono::DateTime<Utc>;
type Long = i32;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/fee_transfers.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone"
)]
pub struct FeeTransfersQuery;

#[allow(clippy::derivable_impls)]
impl Default for fee_transfers_query::BlockQueryInput {
    fn default() -> Self {
        fee_transfers_query::BlockQueryInput {
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

#[allow(clippy::derivable_impls)]
impl Default for FeetransferQueryInput {
    fn default() -> Self {
        FeetransferQueryInput {
            date_time_nin: None,
            recipient: None,
            block_state_hash: None,
            recipient_lt: None,
            type_lte: None,
            fee_gte: None,
            fee_nin: None,
            block_height_in: None,
            type_: None,
            date_time_gte: None,
            type_in: None,
            block_height_ne: None,
            date_time: None,
            fee_ne: None,
            block_height_exists: None,
            or: None,
            type_gte: None,
            canonical_exists: None,
            date_time_lt: None,
            fee_exists: None,
            date_time_gt: None,
            type_lt: None,
            block_height_gte: None,
            recipient_lte: None,
            block_height_nin: None,
            block_height_gt: None,
            block_height: None,
            block_height_lt: None,
            canonical: None,
            type_gt: None,
            block_state_hash_exists: None,
            canonical_ne: None,
            fee_gt: None,
            recipient_gte: None,
            and: None,
            fee_lt: None,
            type_exists: None,
            recipient_in: None,
            recipient_exists: None,
            block_height_lte: None,
            date_time_in: None,
            date_time_exists: None,
            fee_lte: None,
            date_time_lte: None,
            recipient_nin: None,
            type_nin: None,
            fee: None,
            fee_in: None,
            type_ne: None,
            date_time_ne: None,
            recipient_ne: None,
            recipient_gt: None,
        }
    }
}
