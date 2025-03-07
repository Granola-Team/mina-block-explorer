use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/token_holders.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone,Default",
    skip_serializing_none
)]
pub struct TokenHoldersQuery;
