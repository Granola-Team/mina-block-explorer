use self::account_activity_query::{
    AccountActivityQueryBlocks, AccountActivityQueryBlocksTransactions,
    AccountActivityQueryIncomingTransactions, AccountActivityQueryIncomingTransactionsBlock,
    AccountActivityQueryOutgoingTransactions, AccountActivityQueryOutgoingTransactionsBlock,
    AccountActivityQuerySnarks, AccountActivityQuerySnarksBlock, BlockCreatorAccountQueryInput,
    BlockQueryInput, SnarkQueryInput, TransactionQueryInput,
};
use crate::{
    account_activity::graphql::account_activity_query::{
        BlockProtocolStateConsensusStateQueryInput, BlockProtocolStateQueryInput,
    },
    blocks::graphql::blocks_query::{BlocksQueryBlocks, BlocksQueryBlocksTransactions},
    snarks::graphql::snarks_query::{SnarksQuerySnarks, SnarksQuerySnarksBlock},
    user_commands::graphql::transactions_query::{
        TransactionsQueryTransactions, TransactionsQueryTransactionsBlock,
    },
};
use chrono::Utc;
use graphql_client::GraphQLQuery;
type DateTime = chrono::DateTime<Utc>;
type Long = i32;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/account_dialog.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone,Default",
    skip_serializing_none
)]
pub struct AccountActivityQuery;

impl From<AccountActivityQueryIncomingTransactions> for TransactionsQueryTransactions {
    fn from(item: AccountActivityQueryIncomingTransactions) -> Self {
        TransactionsQueryTransactions {
            fee: item.fee,
            from: item.from,
            hash: item.hash,
            to: item.to,
            amount: item.amount,
            block: item.block.map(|b| b.into()),
            ..Default::default()
        }
    }
}

impl From<AccountActivityQueryIncomingTransactionsBlock> for TransactionsQueryTransactionsBlock {
    fn from(item: AccountActivityQueryIncomingTransactionsBlock) -> Self {
        TransactionsQueryTransactionsBlock {
            date_time: item.date_time,
            ..Default::default()
        }
    }
}

impl From<AccountActivityQueryOutgoingTransactions> for TransactionsQueryTransactions {
    fn from(item: AccountActivityQueryOutgoingTransactions) -> Self {
        TransactionsQueryTransactions {
            fee: item.fee,
            from: item.from,
            hash: item.hash,
            to: item.to,
            amount: item.amount,
            block: item.block.map(|b| b.into()),
            ..Default::default()
        }
    }
}

impl From<AccountActivityQueryOutgoingTransactionsBlock> for TransactionsQueryTransactionsBlock {
    fn from(item: AccountActivityQueryOutgoingTransactionsBlock) -> Self {
        TransactionsQueryTransactionsBlock {
            date_time: item.date_time,
            ..Default::default()
        }
    }
}

impl From<AccountActivityQuerySnarks> for SnarksQuerySnarks {
    fn from(item: AccountActivityQuerySnarks) -> Self {
        SnarksQuerySnarks {
            block: item.block.map(|b| b.into()),
            fee: item.fee,
            date_time: item.date_time,
            ..Default::default()
        }
    }
}

impl From<AccountActivityQuerySnarksBlock> for SnarksQuerySnarksBlock {
    fn from(item: AccountActivityQuerySnarksBlock) -> Self {
        SnarksQuerySnarksBlock {
            state_hash: item.state_hash,
        }
    }
}

impl From<AccountActivityQueryBlocks> for BlocksQueryBlocks {
    fn from(item: AccountActivityQueryBlocks) -> Self {
        BlocksQueryBlocks {
            date_time: item.date_time,
            state_hash: item.state_hash,
            transactions: item.transactions.map(|r| r.into()),
            ..Default::default()
        }
    }
}

impl From<AccountActivityQueryBlocksTransactions> for BlocksQueryBlocksTransactions {
    fn from(item: AccountActivityQueryBlocksTransactions) -> Self {
        BlocksQueryBlocksTransactions {
            coinbase: item.coinbase,
            ..Default::default()
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for BlockProtocolStateConsensusStateQueryInput {
    fn default() -> Self {
        BlockProtocolStateConsensusStateQueryInput {
            slot_ne: None,
            block_height_nin: None,
            min_window_density_gt: None,
            block_height_in: None,
            slot_since_genesis_gte: None,
            epoch_count_ne: None,
            slot_lt: None,
            epoch_exists: None,
            or: None,
            blockchain_length_gt: None,
            min_window_density_ne: None,
            total_currency_ne: None,
            slot_since_genesis_lte: None,
            slot_exists: None,
            next_epoch_data: None,
            block_height: None,
            total_currency_gt: None,
            epoch_lt: None,
            epoch_count_gt: None,
            epoch_ne: None,
            blockchain_length_lte: None,
            slot_lte: None,
            slot_nin: None,
            blockchain_length_nin: None,
            slot_in: None,
            min_window_density_gte: None,
            and: None,
            epoch_gt: None,
            slot_since_genesis_nin: None,
            slot_since_genesis_exists: None,
            has_ancestor_in_same_checkpoint_window_exists: None,
            total_currency_lt: None,
            staking_epoch_data: None,
            slot_since_genesis_ne: None,
            slot_gte: None,
            slot: None,
            next_epoch_data_exists: None,
            min_window_density_lt: None,
            blockchain_length_exists: None,
            has_ancestor_in_same_checkpoint_window: None,
            blockchain_length_gte: None,
            epoch: None,
            last_vrf_output_lte: None,
            min_window_density_exists: None,
            epoch_count_nin: None,
            block_height_lte: None,
            total_currency_nin: None,
            block_height_exists: None,
            epoch_count_gte: None,
            blockchain_length_ne: None,
            total_currency_lte: None,
            slot_since_genesis_in: None,
            total_currency_gte: None,
            epoch_nin: None,
            min_window_density_lte: None,
            epoch_count_lte: None,
            slot_gt: None,
            slot_since_genesis_gt: None,
            has_ancestor_in_same_checkpoint_window_ne: None,
            min_window_density_in: None,
            total_currency_in: None,
            total_currency_exists: None,
            min_window_density: None,
            min_window_density_nin: None,
            epoch_gte: None,
            last_vrf_output_gt: None,
            block_height_gte: None,
            blockchain_length_lt: None,
            block_height_gt: None,
            last_vrf_output_nin: None,
            epoch_count: None,
            blockchain_length: None,
            last_vrf_output_exists: None,
            epoch_count_exists: None,
            last_vrf_output_in: None,
            epoch_count_in: None,
            last_vrf_output_ne: None,
            block_height_lt: None,
            slot_since_genesis_lt: None,
            epoch_in: None,
            block_height_ne: None,
            last_vrf_output: None,
            blockchain_length_in: None,
            last_vrf_output_gte: None,
            staking_epoch_data_exists: None,
            epoch_count_lt: None,
            slot_since_genesis: None,
            epoch_lte: None,
            last_vrf_output_lt: None,
            total_currency: None,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for BlockProtocolStateQueryInput {
    fn default() -> Self {
        BlockProtocolStateQueryInput {
            previous_state_hash_exists: None,
            blockchain_state_exists: None,
            consensus_state: None,
            previous_state_hash_ne: None,
            consensus_state_exists: None,
            previous_state_hash_nin: None,
            previous_state_hash_lt: None,
            or: None,
            previous_state_hash_lte: None,
            blockchain_state: None,
            previous_state_hash_gte: None,
            previous_state_hash_gt: None,
            previous_state_hash_in: None,
            and: None,
            previous_state_hash: None,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for BlockQueryInput {
    fn default() -> Self {
        BlockQueryInput {
            creator_nin: None,
            state_hash_lte: None,
            canonical_ne: None,
            date_time_lt: None,
            snark_jobs: None,
            creator_ne: None,
            received_time: None,
            block_height_lte: None,
            state_hash_field_gte: None,
            received_time_in: None,
            block_height: None,
            state_hash_field_exists: None,
            block_height_nin: None,
            creator_gt: None,
            state_hash_gte: None,
            state_hash_lt: None,
            creator_gte: None,
            protocol_state: None,
            canonical_exists: None,
            date_time_nin: None,
            creator_lte: None,
            creator_account: None,
            state_hash_field_lt: None,
            creator_in: None,
            state_hash_ne: None,
            received_time_ne: None,
            creator: None,
            state_hash_field_lte: None,
            date_time_lte: None,
            date_time_exists: None,
            state_hash_field_gt: None,
            date_time: None,
            date_time_gt: None,
            winner_account_exists: None,
            received_time_gte: None,
            protocol_state_exists: None,
            state_hash_exists: None,
            canonical: None,
            creator_exists: None,
            received_time_lte: None,
            block_height_exists: None,
            state_hash_field_ne: None,
            winner_account: None,
            or: None,
            state_hash_field: None,
            received_time_lt: None,
            transactions: None,
            date_time_gte: None,
            and: None,
            creator_account_exists: None,
            block_height_in: None,
            received_time_nin: None,
            snark_jobs_nin: None,
            date_time_in: None,
            snark_jobs_in: None,
            block_height_lt: None,
            state_hash_field_in: None,
            block_height_ne: None,
            transactions_exists: None,
            creator_lt: None,
            received_time_exists: None,
            block_height_gt: None,
            state_hash_nin: None,
            state_hash_field_nin: None,
            date_time_ne: None,
            state_hash_in: None,
            state_hash: None,
            block_height_gte: None,
            received_time_gt: None,
            snark_jobs_exists: None,
            state_hash_gt: None,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for BlockCreatorAccountQueryInput {
    fn default() -> Self {
        BlockCreatorAccountQueryInput {
            public_key_exists: None,
            public_key_ne: None,
            public_key_lte: None,
            or: None,
            public_key_gte: None,
            public_key_lt: None,
            public_key_in: None,
            and: None,
            public_key: None,
            public_key_nin: None,
            public_key_gt: None,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for SnarkQueryInput {
    fn default() -> Self {
        SnarkQueryInput {
            block_height_lt: None,
            fee_in: None,
            prover_gte: None,
            fee_lt: None,
            date_time_nin: None,
            prover_in: None,
            prover_ne: None,
            block_height_gt: None,
            prover: None,
            block_exists: None,
            date_time_lte: None,
            date_time: None,
            fee_gt: None,
            date_time_exists: None,
            work_ids_exists: None,
            prover_lte: None,
            date_time_in: None,
            block_height_nin: None,
            fee_exists: None,
            canonical: None,
            canonical_ne: None,
            date_time_gt: None,
            block_height_exists: None,
            block: None,
            block_height_ne: None,
            block_height_gte: None,
            work_ids_in: None,
            fee: None,
            fee_gte: None,
            prover_nin: None,
            and: None,
            prover_exists: None,
            or: None,
            date_time_ne: None,
            work_ids: None,
            date_time_lt: None,
            block_height_in: None,
            fee_nin: None,
            work_ids_nin: None,
            prover_lt: None,
            prover_gt: None,
            block_height_lte: None,
            fee_lte: None,
            date_time_gte: None,
            fee_ne: None,
            block_height: None,
            canonical_exists: None,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for TransactionQueryInput {
    fn default() -> Self {
        TransactionQueryInput {
            fee_in: None,
            canonical_exists: None,
            memo_lt: None,
            from_account: None,
            memo_gte: None,
            fee_gt: None,
            to_account_exists: None,
            kind_lte: None,
            fee_token_in: None,
            token_lt: None,
            fee_exists: None,
            memo_gt: None,
            token_nin: None,
            token_gte: None,
            canonical_ne: None,
            hash_gt: None,
            receiver_exists: None,
            failure_reason_exists: None,
            date_time_exists: None,
            nonce_nin: None,
            fee_token_gte: None,
            id_in: None,
            is_delegation_exists: None,
            fee_payer: None,
            date_time_ne: None,
            kind_gt: None,
            amount_ne: None,
            to_gte: None,
            fee_payer_exists: None,
            kind_lt: None,
            id_lt: None,
            hash_ne: None,
            to_nin: None,
            date_time_nin: None,
            block_height_exists: None,
            nonce_lte: None,
            fee_token_nin: None,
            id: None,
            fee_token: None,
            to_account: None,
            block_height_lte: None,
            and: None,
            amount: None,
            fee: None,
            fee_token_lt: None,
            nonce_gt: None,
            amount_gt: None,
            receiver: None,
            hash_gte: None,
            token_ne: None,
            to_exists: None,
            source: None,
            fee_lt: None,
            fee_gte: None,
            hash_lt: None,
            amount_gte: None,
            hash_exists: None,
            from: None,
            failure_reason_ne: None,
            id_gte: None,
            kind_exists: None,
            block_height_gte: None,
            fee_ne: None,
            amount_lte: None,
            from_lte: None,
            failure_reason_lte: None,
            memo_ne: None,
            hash: None,
            nonce_ne: None,
            failure_reason_lt: None,
            from_in: None,
            block_height_nin: None,
            id_ne: None,
            amount_nin: None,
            kind_gte: None,
            from_gte: None,
            from_nin: None,
            is_delegation: None,
            nonce_lt: None,
            from_account_exists: None,
            to_gt: None,
            token: None,
            failure_reason_in: None,
            kind_ne: None,
            token_exists: None,
            id_nin: None,
            fee_token_ne: None,
            date_time_gte: None,
            to_in: None,
            block_exists: None,
            date_time_lt: None,
            from_exists: None,
            kind_nin: None,
            to_ne: None,
            block_height: None,
            failure_reason_gt: None,
            id_gt: None,
            date_time_lte: None,
            block_height_ne: None,
            hash_nin: None,
            to_lte: None,
            nonce: None,
            memo_in: None,
            fee_token_exists: None,
            fee_token_gt: None,
            memo: None,
            from_gt: None,
            failure_reason_nin: None,
            token_gt: None,
            fee_nin: None,
            kind_in: None,
            canonical: None,
            fee_lte: None,
            or: None,
            kind: None,
            memo_exists: None,
            from_lt: None,
            date_time_in: None,
            source_exists: None,
            hash_lte: None,
            id_lte: None,
            hash_in: None,
            block_height_gt: None,
            amount_lt: None,
            block_height_lt: None,
            amount_in: None,
            failure_reason: None,
            memo_nin: None,
            nonce_exists: None,
            failure_reason_gte: None,
            fee_token_lte: None,
            token_lte: None,
            is_delegation_ne: None,
            date_time: None,
            memo_lte: None,
            block: None,
            date_time_gt: None,
            from_ne: None,
            nonce_in: None,
            id_exists: None,
            block_height_in: None,
            amount_exists: None,
            nonce_gte: None,
            token_in: None,
            to_lt: None,
            to: None,
        }
    }
}
