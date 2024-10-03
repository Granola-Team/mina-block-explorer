// use chrono::Utc;
use self::staking_ledgers_query::StakeQueryInput;
use graphql_client::GraphQLQuery;

// type DateTime = chrono::DateTime<Utc>;
// type Long = i32;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/staking_ledgers.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone,Default",
    skip_serializing_none
)]
pub struct StakingLedgersQuery;

#[allow(clippy::derivable_impls)]
impl Default for StakeQueryInput {
    fn default() -> Self {
        StakeQueryInput {
            username: None,
            stake_lte: None,
            timing_exists: None,
            chain_id_gte: None,
            balance_ne: None,
            nonce: None,
            delegate_gte: None,
            epoch_nin: None,
            public_key_gt: None,
            ledger_hash_nin: None,
            ledger_hash_lt: None,
            chain_id_in: None,
            delegate_nin: None,
            token_nin: None,
            token_lt: None,
            pk_gte: None,
            balance_gt: None,
            ledger_hash_lte: None,
            epoch_in: None,
            ledger_hash_gt: None,
            delegate_gt: None,
            pk_nin: None,
            balance_lte: None,
            public_key_nin: None,
            ledger_hash_ne: None,
            voting_for: None,
            nonce_in: None,
            permissions_exists: None,
            permissions: None,
            chain_id_nin: None,
            token_gte: None,
            chain_id_gt: None,
            nonce_lt: None,
            pk_gt: None,
            voting_for_lte: None,
            receipt_chain_hash_in: None,
            public_key_gte: None,
            ledger_hash: None,
            chain_id_lte: None,
            ledger_hash_in: None,
            receipt_chain_hash_lt: None,
            voting_for_gt: None,
            balance_lt: None,
            token_ne: None,
            pk_in: None,
            balance_exists: None,
            voting_for_gte: None,
            delegate: None,
            or: None,
            balance_nin: None,
            epoch_ne: None,
            nonce_gte: None,
            and: None,
            voting_for_exists: None,
            public_key_exists: None,
            token_in: None,
            receipt_chain_hash_lte: None,
            epoch_lte: None,
            balance_gte: None,
            voting_for_lt: None,
            chain_id: None,
            delegate_ne: None,
            public_key_in: None,
            delegate_lte: None,
            pk_lte: None,
            receipt_chain_hash_ne: None,
            voting_for_in: None,
            public_key_lte: None,
            receipt_chain_hash: None,
            voting_for_ne: None,
            token: None,
            public_key: None,
            balance_in: None,
            public_key_lt: None,
            chain_id_exists: None,
            token_lte: None,
            delegate_exists: None,
            balance: None,
            receipt_chain_hash_gte: None,
            epoch_exists: None,
            nonce_nin: None,
            chain_id_ne: None,
            epoch_gte: None,
            epoch_gt: None,
            voting_for_nin: None,
            pk: None,
            pk_ne: None,
            nonce_exists: None,
            receipt_chain_hash_exists: None,
            token_exists: None,
            epoch: None,
            receipt_chain_hash_gt: None,
            chain_id_lt: None,
            token_gt: None,
            nonce_gt: None,
            receipt_chain_hash_nin: None,
            timing: None,
            pk_lt: None,
            public_key_ne: None,
            nonce_ne: None,
            epoch_lt: None,
            nonce_lte: None,
            delegate_in: None,
            ledger_hash_gte: None,
            ledger_hash_exists: None,
            pk_exists: None,
            delegate_lt: None,
        }
    }
}
