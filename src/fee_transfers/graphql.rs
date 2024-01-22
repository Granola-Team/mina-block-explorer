use chrono::Utc;
use graphql_client::GraphQLQuery;

type DateTime = chrono::DateTime<Utc>;
type Long = i32;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/fee_transfers.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone"
)]
pub struct FeeTransfersQuery;
