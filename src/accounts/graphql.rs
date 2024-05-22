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
