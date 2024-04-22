use crate::internal_commands::graphql::internal_commands_query::FeetransferQueryInput;
use chrono::Utc;
use graphql_client::GraphQLQuery;

type DateTime = chrono::DateTime<Utc>;
type Long = i32;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/internal_commands.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone,Default",
    skip_serializing_none
)]
pub struct InternalCommandsQuery;

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
