use graphql_client::reqwest::post_graphql;
use crate::api_models::MyError;
use super::graphql::*;

pub async fn load_data(public_key: Option<String>) -> Result<snarks_query::ResponseData, MyError> {
    let url = "https://graphql.minaexplorer.com";
    let variables = snarks_query::Variables {
        sort_by: snarks_query::SnarkSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(25),
        query: snarks_query::SnarkQueryInput {
            prover: public_key,
            canonical: Some(true),
            ..Default::default()
        }
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<SnarksQuery, _>(&client, url, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response.data.ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}