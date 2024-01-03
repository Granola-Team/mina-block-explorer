use chrono::Utc;
use graphql_client::GraphQLQuery;

type DateTime = chrono::DateTime<Utc>;
type Long = i32;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/transactions.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone,Default"
)]
pub struct TransactionsQuery;
