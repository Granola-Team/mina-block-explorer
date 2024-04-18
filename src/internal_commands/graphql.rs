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
