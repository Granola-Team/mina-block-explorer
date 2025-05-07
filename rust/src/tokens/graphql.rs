use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schemas/mina-explorer.graphql",
    query_path = "./graphql/queries/tokens.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone,Default",
    skip_serializing_none
)]
pub struct TokensQuery;

#[allow(clippy::derivable_impls)]
impl Default for tokens_query::TokensQueryInput {
    fn default() -> Self {
        tokens_query::TokensQueryInput {
            token: None,
            owner: None,
            symbol: None,
            supply: None,
        }
    }
}
