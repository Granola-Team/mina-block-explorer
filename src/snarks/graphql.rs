use chrono::Utc;
use graphql_client::GraphQLQuery;

use self::snarks_query::SnarkQueryInput;

type DateTime = chrono::DateTime<Utc>;
type Long = i32;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/snarks.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone"
)]
pub struct SnarksQuery;

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