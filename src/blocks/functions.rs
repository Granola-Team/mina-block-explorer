use super::{
    graphql::{
        blocks_query::{BlocksQueryBlocks, BlocksQueryBlocksSnarkJobs},
        *,
    },
    models::BlocksQueryBlocksTransactionsUserCommandsExt,
};
use crate::common::{
    constants::*,
    functions::{format_number, nanomina_str_to_mina, nanomina_to_mina},
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
pub fn get_snark_fee(snark: &BlocksQueryBlocksSnarkJobs) -> String {
    snark
        .fee
        .map(|i| nanomina_to_mina(i as u64))
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_user_commands(
    block: &BlocksQueryBlocks,
) -> Option<Vec<Option<BlocksQueryBlocksTransactionsUserCommandsExt>>> {
    let block_state_hash = block.state_hash.clone();
    block.transactions.as_ref().and_then(|txn| {
        txn.user_commands.as_ref().map(|uc| {
            uc.iter()
                .filter_map(|uc| uc.clone())
                .map(|t| {
                    Some(BlocksQueryBlocksTransactionsUserCommandsExt {
                        from: t.from,
                        to: t.to,
                        hash: t.hash,
                        fee: t.fee,
                        amount: t.amount,
                        kind: t.kind,
                        memo: t.memo,
                        failure_reason: t.failure_reason,
                        nonce: t.nonce,
                        block_state_hash: block_state_hash.clone(),
                    })
                })
                .collect::<Vec<_>>()
        })
    })
}

pub fn get_block_height(block: &BlocksQueryBlocks) -> String {
    block
        .block_height
        .map_or_else(String::new, |o| format_number(o.to_string()))
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

pub fn get_internal_command_count(block: &BlocksQueryBlocks) -> Option<usize> {
    Some(block.transactions.as_ref().map_or(0, |txn| {
           txn.fee_transfer.as_ref().map_or(0, |fee_transfers| fee_transfers.len())
               + txn.coinbase.is_some() as usize
       }))
}

pub fn get_slot(block: &BlocksQueryBlocks) -> String {
    block.protocol_state.as_ref().map_or_else(String::new, |o| {
        o.consensus_state.as_ref().map_or_else(String::new, |o| {
            o.slot_since_genesis
                .map_or_else(String::new, |o| format_number(o.to_string()))
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

pub fn get_global_slot(block: &BlocksQueryBlocks) -> String {
    block
        .protocol_state
        .as_ref()
        .and_then(|o| o.consensus_state.as_ref())
        .and_then(|o| o.slot_since_genesis)
        .map_or_else(String::new, |o| format_number(o.to_string()))
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
    limit: Option<u64>,
    block_creator_account: Option<String>,
    state_hash: Option<String>,
    block_height: Option<u64>,
    slot: Option<u64>,
    canonical: Option<bool>,
) -> Result<blocks_query::ResponseData, MyError> {
    let variables = blocks_query::Variables {
        sort_by: blocks_query::BlockSortByInput::BLOCKHEIGHT_DESC,
        limit: limit.map_or(Some(25i64), |l| Some(l as i64)),
        query: blocks_query::BlockQueryInput {
            canonical,
            state_hash,
            block_height_lte: block_height.map(|x| x as i64),
            creator_account: Some(blocks_query::BlockCreatorAccountQueryInput {
                public_key: block_creator_account,
                ..Default::default()
            }),
            protocol_state: Some(blocks_query::BlockProtocolStateQueryInput {
                consensus_state: Some(blocks_query::BlockProtocolStateConsensusStateQueryInput {
                    slot_since_genesis_lte: slot.map(|x| x as i64),
                    ..Default::default()
                }),
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
