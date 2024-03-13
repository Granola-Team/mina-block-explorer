use self::snarks_query::SnarkQueryInput;
use chrono::Utc;
use graphql_client::GraphQLQuery;

type DateTime = chrono::DateTime<Utc>;
type Long = i32;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/snarks.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone",
    skip_serializing_none
)]
pub struct SnarksQuery;

#[allow(clippy::derivable_impls)]
impl Default for snarks_query::SnarksQuerySnarks {
    fn default() -> Self {
        snarks_query::SnarksQuerySnarks {
            block_height: None,
            date_time: None,
            prover: None,
            work_ids: None,
            block: None,
            fee: None,
            canonical: None,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for snarks_query::SnarksQuerySnarksBlock {
    fn default() -> Self {
        snarks_query::SnarksQuerySnarksBlock { state_hash: None }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for snarks_query::BlockQueryInput {
    fn default() -> Self {
        snarks_query::BlockQueryInput {
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
impl Default for SnarkQueryInput {
    fn default() -> Self {
        SnarkQueryInput {
            block_height_lt: None,
            fee_in: None,
            prover_gte: None,
            fee_lt: None,
            date_time_nin: None,
            prover_in: None,
            prover_ne: None,
            block_height_gt: None,
            prover: None,
            block_exists: None,
            date_time_lte: None,
            date_time: None,
            fee_gt: None,
            date_time_exists: None,
            work_ids_exists: None,
            prover_lte: None,
            date_time_in: None,
            block_height_nin: None,
            fee_exists: None,
            canonical: None,
            canonical_ne: None,
            date_time_gt: None,
            block_height_exists: None,
            block: None,
            block_height_ne: None,
            block_height_gte: None,
            work_ids_in: None,
            fee: None,
            fee_gte: None,
            prover_nin: None,
            and: None,
            prover_exists: None,
            or: None,
            date_time_ne: None,
            work_ids: None,
            date_time_lt: None,
            block_height_in: None,
            fee_nin: None,
            work_ids_nin: None,
            prover_lt: None,
            prover_gt: None,
            block_height_lte: None,
            fee_lte: None,
            date_time_gte: None,
            fee_ne: None,
            block_height: None,
            canonical_exists: None,
        }
    }
}
