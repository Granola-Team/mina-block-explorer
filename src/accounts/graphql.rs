// use self::accounts_query::AccountQueryInput;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/accounts.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone",
    skip_serializing_none
)]
pub struct AccountsQuery;

#[allow(clippy::derivable_impls)]
impl Default for accounts_query::AccountQueryInput {
    fn default() -> Self {
        accounts_query::AccountQueryInput {
            public_key: None,
            username: None,
            balance_lte: None,
            delegate: None,
            zkapp: None,
            token: None,
        }
    }
}
