use graphql_client::reqwest::post_graphql;

use crate::common::models::MyError;
use super::graphql::*;

pub async fn load_data(limit: i64, public_key: Option<String>) -> Result<blocks_query::ResponseData, MyError> {
    let url = "https://graphql.minaexplorer.com";
    let variables = blocks_query::Variables {
        sort_by: blocks_query::BlockSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(limit),
        query: blocks_query::BlockQueryInput {
            canonical: Some(true),
            creator_account: Some(blocks_query::BlockCreatorAccountQueryInput{ 
                public_key: public_key, 
                ..Default::default()
            }),
            ..Default::default()
            
        },
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<BlocksQuery, _>(&client, url, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}