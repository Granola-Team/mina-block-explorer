use chrono::Utc;
use graphql_client::GraphQLQuery;

type DateTime = chrono::DateTime<Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/snarks.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone"
)]
pub struct SnarksQuery;
