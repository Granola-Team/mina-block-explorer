use super::graphql::{
    blocks_query::{
        BlocksQueryBlocks, BlocksQueryBlocksSnarkJobs, BlocksQueryBlocksTransactionsUserCommands,
    },
    *,
};
use crate::common::{
    constants::GRAPHQL_ENDPOINT,
    functions::{nanomina_str_to_mina, nanomina_to_mina},
    models::MyError,
};
use graphql_client::reqwest::post_graphql;

pub fn get_snark_block_state_hash(snark: &BlocksQueryBlocksSnarkJobs) -> String {
    snark
        .block_state_hash
        .as_ref()
        .map_or_else(String::new, |o| o.to_string())
}
pub fn get_snark_date_time(snark: &BlocksQueryBlocksSnarkJobs) -> String {
    snark.date_time.map_or_else(String::new, |o| o.to_string())
}
pub fn get_snark_prover(snark: &BlocksQueryBlocksSnarkJobs) -> String {
    snark
        .prover
        .as_ref()
        .map_or_else(String::new, |o| o.to_string())
}
pub fn get_snark_work_ids(snark: &BlocksQueryBlocksSnarkJobs) -> Vec<String> {
    snark.work_ids.as_ref().map_or_else(Vec::new, |ids| {
        ids.iter()
            .map(|id| id.map_or_else(String::new, |id| id.to_string()))
            .collect::<Vec<_>>()
    })
}
pub fn get_snark_fee(snark: &BlocksQueryBlocksSnarkJobs) -> String {
    snark
        .fee
        .map(|i| nanomina_to_mina(i as u64))
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_user_commands(
    block: &BlocksQueryBlocks,
) -> Option<Vec<Option<BlocksQueryBlocksTransactionsUserCommands>>> {
    block
        .transactions
        .as_ref()
        .and_then(|t| t.user_commands.clone())
}

pub fn get_user_command_from(uc: &BlocksQueryBlocksTransactionsUserCommands) -> String {
    uc.from.as_ref().map_or("".to_string(), |o| o.to_string())
}

pub fn get_user_command_to(uc: &BlocksQueryBlocksTransactionsUserCommands) -> String {
    uc.to.as_ref().map_or("".to_string(), |o| o.to_string())
}

pub fn get_user_command_hash(uc: &BlocksQueryBlocksTransactionsUserCommands) -> String {
    uc.hash.as_ref().map_or("".to_string(), |o| o.to_string())
}

pub fn get_user_command_fee(uc: &BlocksQueryBlocksTransactionsUserCommands) -> String {
    uc.fee
        .map(|f| f.round() as u64)
        .map(nanomina_to_mina)
        .unwrap_or_default()
}

pub fn get_user_command_amount(uc: &BlocksQueryBlocksTransactionsUserCommands) -> String {
    uc.amount
        .map(|f| f.round() as u64)
        .map(nanomina_to_mina)
        .unwrap_or_default()
}

pub fn get_block_height(block: &BlocksQueryBlocks) -> String {
    block
        .block_height
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_canonical(block: &BlocksQueryBlocks) -> Option<bool> {
    block.canonical
}

pub fn get_date_time(block: &BlocksQueryBlocks) -> String {
    block.date_time.map_or_else(String::new, |o| o.to_string())
}

pub fn get_creator_account(block: &BlocksQueryBlocks) -> String {
    block
        .creator_account
        .as_ref()
        .map_or_else(String::new, |o| {
            o.public_key
                .as_ref()
                .map_or_else(String::new, |o1| o1.to_string())
        })
}

pub fn get_coinbase(block: &BlocksQueryBlocks) -> String {
    block
        .transactions
        .as_ref()
        .and_then(|o| o.coinbase.as_deref())
        .map(nanomina_str_to_mina)
        .unwrap_or_default()
}

pub fn get_transaction_count(block: &BlocksQueryBlocks) -> Option<usize> {
    block
        .transactions
        .as_ref()
        .and_then(|o| o.user_commands.as_ref().map(|o1| o1.len()))
}

pub fn get_snark_job_count(block: &BlocksQueryBlocks) -> Option<usize> {
    block.snark_jobs.as_ref().map(|o| o.len())
}

pub fn get_fee_transfer_count(block: &BlocksQueryBlocks) -> Option<usize> {
    block
        .transactions
        .as_ref()
        .and_then(|o| o.fee_transfer.as_ref().map(|o1| o1.len()))
}

pub fn get_slot(block: &BlocksQueryBlocks) -> String {
    block.protocol_state.as_ref().map_or_else(String::new, |o| {
        o.consensus_state.as_ref().map_or_else(String::new, |o| {
            o.slot.map_or_else(String::new, |o| o.to_string())
        })
    })
}

pub fn get_state_hash(block: &BlocksQueryBlocks) -> String {
    block
        .state_hash
        .as_ref()
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_snarked_ledger_hash(block: &BlocksQueryBlocks) -> String {
    block
        .protocol_state
        .as_ref()
        .and_then(|o| o.blockchain_state.as_ref())
        .and_then(|o1| o1.snarked_ledger_hash.as_ref())
        .map_or_else(|| "".to_string(), ToString::to_string)
}

pub fn get_winning_account(block: &BlocksQueryBlocks) -> String {
    block
        .winner_account
        .as_ref()
        .and_then(|o| o.public_key.as_ref())
        .map_or_else(|| "".to_string(), ToString::to_string)
}

pub fn get_global_slot(block: &BlocksQueryBlocks) -> String {
    block
        .protocol_state
        .as_ref()
        .and_then(|o| o.consensus_state.as_ref())
        .and_then(|o| o.slot_since_genesis)
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_epoch(block: &BlocksQueryBlocks) -> String {
    block
        .protocol_state
        .as_ref()
        .and_then(|o| o.consensus_state.as_ref())
        .and_then(|o| o.epoch)
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_previous_state_hash(block: &BlocksQueryBlocks) -> String {
    block
        .protocol_state
        .as_ref()
        .and_then(|o| o.previous_state_hash.as_ref())
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_staged_ledger_hash(block: &BlocksQueryBlocks) -> String {
    block
        .protocol_state
        .as_ref()
        .and_then(|o| o.blockchain_state.as_ref())
        .and_then(|o1| o1.staged_ledger_hash.as_ref())
        .map_or_else(|| "".to_string(), ToString::to_string)
}

pub fn get_transaction_fees(block: &BlocksQueryBlocks) -> String {
    block
        .tx_fees
        .as_deref()
        .map(nanomina_str_to_mina)
        .unwrap_or_default()
}

pub fn get_winner_total(block: &BlocksQueryBlocks) -> String {
    block
        .winner_account
        .as_ref()
        .and_then(|w| w.balance.as_ref())
        .and_then(|b| b.total.as_deref())
        .map_or(String::new(), nanomina_str_to_mina)
}

pub fn get_snark_fees(block: &BlocksQueryBlocks) -> String {
    block
        .snark_fees
        .as_deref()
        .map(nanomina_str_to_mina)
        .unwrap_or_default()
}

pub fn get_total_currency(block: &BlocksQueryBlocks) -> String {
    block
        .protocol_state
        .as_ref()
        .and_then(|o| o.consensus_state.as_ref())
        .and_then(|o| o.total_currency)
        .map(|f| f.round() as u64)
        .map(nanomina_to_mina)
        .unwrap_or_default()
}

pub fn get_coinbase_receiver(block: &BlocksQueryBlocks) -> String {
    block.transactions.as_ref().map_or_else(String::new, |o| {
        o.coinbase_receiver_account
            .as_ref()
            .map_or_else(String::new, |o| {
                o.public_key
                    .as_ref()
                    .map_or_else(String::new, |o| o.to_string())
            })
    })
}

pub async fn load_data(
    limit: i64,
    public_key: Option<String>,
    state_hash: Option<String>,
    block_height: Option<i64>,
    canonical: Option<bool>,
) -> Result<blocks_query::ResponseData, MyError> {
    let variables = blocks_query::Variables {
        sort_by: blocks_query::BlockSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(limit),
        query: blocks_query::BlockQueryInput {
            canonical,
            state_hash,
            block_height,
            creator_account: Some(blocks_query::BlockCreatorAccountQueryInput {
                public_key,
                ..Default::default()
            }),
            ..Default::default()
        },
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<BlocksQuery, _>(&client, GRAPHQL_ENDPOINT, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}
