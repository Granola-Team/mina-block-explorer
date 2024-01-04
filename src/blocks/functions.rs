use graphql_client::reqwest::post_graphql;

use super::graphql::{blocks_query::BlocksQueryBlocks, *};
use crate::common::models::MyError;

pub fn get_block_height(block: &BlocksQueryBlocks) -> String {
    block
        .block_height
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_date_time(block: &BlocksQueryBlocks) -> String {
    block.date_time.map_or_else(String::new, |o| o.to_string())
}

pub fn get_creator_account(block: &BlocksQueryBlocks) -> String {
    block.creator_account.as_ref().map_or_else(String::new, |o| {
        o.public_key.as_ref().map_or_else(String::new, |o1| o1.to_string())
    })
}

pub fn get_coinbase(block: &BlocksQueryBlocks) -> String {
    block.transactions.as_ref().map_or_else(String::new, |o| {
        o.coinbase.map_or_else(String::new, |o1| o1.to_string())
    })
}

pub fn get_transaction_count(block: &BlocksQueryBlocks) -> String {
    block.transactions.as_ref().map_or_else(String::new, |o| {
        o.user_commands.as_ref()
            .map_or_else(String::new, |o1| o1.len().to_string())
    })
}

pub fn get_snark_job_count(block: &BlocksQueryBlocks) -> String {
    block
        .snark_jobs.as_ref()
        .map_or_else(String::new, |o| o.len().to_string())
}

pub fn get_slot(block: &BlocksQueryBlocks) -> String {
    block.protocol_state.as_ref().map_or_else(String::new, |o| {
        o.consensus_state.as_ref().map_or_else(String::new, |o| {
            o.slot.map_or_else(String::new, |o| o.to_string())
        })
    })
}

pub fn get_state_hash(block: &BlocksQueryBlocks) -> String {
    block.state_hash.as_ref().map_or_else(String::new, |o| o.to_string())
    //         block.transactions.coinbase_receiver_account.public_key.to_string(),
}

pub fn get_coinbase_receiver(block: &BlocksQueryBlocks) -> String {
    block.transactions.as_ref().map_or_else(String::new, |o| {
        o.coinbase_receiver_account.as_ref().map_or_else(String::new, |o| {
            o.public_key.as_ref().map_or_else(String::new, |o| o.to_string())
        })
    })
}

pub async fn load_data(
    limit: i64,
    public_key: Option<String>,
) -> Result<blocks_query::ResponseData, MyError> {
    let url = "https://graphql.minaexplorer.com";
    let variables = blocks_query::Variables {
        sort_by: blocks_query::BlockSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(limit),
        query: blocks_query::BlockQueryInput {
            canonical: Some(true),
            creator_account: Some(blocks_query::BlockCreatorAccountQueryInput {
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
