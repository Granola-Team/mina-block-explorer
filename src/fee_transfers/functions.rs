use super::graphql::FeeTransfersQuery;
use crate::{
    common::{functions::nanomina_to_mina, models::MyError},
    fee_transfers::graphql::{
        fee_transfers_query, fee_transfers_query::FeeTransfersQueryFeetransfers,
    },
};
use graphql_client::reqwest::post_graphql;

pub fn get_receipient(fee_transfer: &FeeTransfersQueryFeetransfers) -> String {
    fee_transfer
        .recipient
        .as_ref()
        .map_or("".to_string(), |o| o.to_string())
}

pub fn get_fee(fee_transfer: &FeeTransfersQueryFeetransfers) -> String {
    fee_transfer
        .fee
        .map(|i| nanomina_to_mina(i as f64))
        .unwrap_or_default()
}

pub fn get_type(fee_transfer: &FeeTransfersQueryFeetransfers) -> String {
    fee_transfer
        .type_
        .as_ref()
        .map_or("".to_string(), |o| o.to_string())
}

pub fn get_date_time(fee_transfer: &FeeTransfersQueryFeetransfers) -> String {
    fee_transfer
        .date_time
        .map_or("".to_string(), |o| o.to_string())
}

pub async fn load_data(
    limit: i64,
    state_hash: Option<String>,
) -> Result<fee_transfers_query::ResponseData, MyError> {
    let url = "https://graphql.minaexplorer.com";
    let variables = fee_transfers_query::Variables {
        sort_by: fee_transfers_query::FeetransferSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(limit),
        query: fee_transfers_query::FeetransferQueryInput {
            block_state_hash: Some(fee_transfers_query::BlockQueryInput {
                state_hash,
                ..Default::default()
            }),
            ..Default::default()
        },
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<FeeTransfersQuery, _>(&client, url, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}
