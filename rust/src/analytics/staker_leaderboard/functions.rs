use graphql_client::reqwest::post_graphql;

use crate::common::{constants::GRAPHQL_ENDPOINT, models::MyError};

use super::graphql::{
    TopStakersQuery,
    top_stakers_query::{
        BlockProtocolStateConsensusStateQueryInput, BlockProtocolStateQueryInput, BlockQueryInput,
        ResponseData, TopStakersQueryInput, TopStakersSortByInput, Variables,
    },
};

pub async fn load_data(
    epoch: Option<u32>,
    sort_by: Option<TopStakersSortByInput>,
) -> Result<ResponseData, MyError> {
    if epoch.is_none() {
        return Err(MyError::ParseError("Epoch must not be None".into()));
    }

    let query = TopStakersQueryInput {
        epoch: epoch
            .map(|e| e as i64)
            .expect("Expected epoch to be present"),
    };

    let variables = Variables {
        limit: Some(50),
        blocks_query: BlockQueryInput {
            protocol_state: Some(BlockProtocolStateQueryInput {
                consensus_state: Some(BlockProtocolStateConsensusStateQueryInput {
                    epoch: epoch.map(|e| e as i64),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
        query,
        sort_by: sort_by.unwrap_or(TopStakersSortByInput::NUM_CANONICAL_BLOCKS_PRODUCED_DESC),
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<TopStakersQuery, _>(&client, GRAPHQL_ENDPOINT, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}
