use super::graphql::transactions_query::{self, TransactionsQueryTransactions};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PooledUserCommandSource {
    #[serde(rename = "publicKey")]
    pub public_key: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PoolUserCommandReceiver {
    #[serde(rename = "publicKey")]
    pub public_key: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PooledUserCommand {
    pub hash: Option<String>,
    pub kind: Option<String>,
    pub nonce: Option<u64>,
    pub source: Option<PooledUserCommandSource>,
    pub receiver: Option<PoolUserCommandReceiver>,
    pub amount: Option<String>,
    pub fee: Option<String>,
    pub memo: Option<String>,
    #[serde(rename = "fee_token")]
    pub fee_token: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PooledUserCommandsResponse {
    pub data: PooledUserCommands,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PooledUserCommands {
    #[serde(rename = "pooledUserCommands")]
    pub pooled_user_commands: Vec<PooledUserCommand>,
}

impl From<PooledUserCommand> for transactions_query::TransactionsQueryTransactions {
    fn from(txn: PooledUserCommand) -> Self {
        transactions_query::TransactionsQueryTransactions {
            zkapp: None,
            block_height: None,
            failure_reason: None,
            canonical: None,
            amount: if let Some(amount) = txn.amount {
                amount.parse().ok()
            } else {
                None
            },
            fee: if let Some(fee) = txn.fee {
                fee.parse().ok()
            } else {
                None
            },
            kind: txn.kind,
            to: match txn.receiver.as_ref() {
                Some(receiver) => receiver.public_key.clone(),
                None => None,
            },
            from: match txn.source {
                Some(source) => source.public_key,
                None => None,
            },
            sender_username: None,
            nonce: txn.nonce.map(|x| x as i64),
            memo: txn.memo,
            hash: txn.hash,
            block: None,
            receiver: txn.receiver.as_ref().and_then(|r| r.public_key.clone()),
            receiver_account: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PendingTxnParty {
    #[serde(rename = "publicKey")]
    pub public_key: Option<String>,
}

#[derive(Clone)]
pub struct PendingTxn {
    pub txn_hash: Option<String>,
    pub kind: Option<String>,
    pub sender_username: Option<String>,
    pub nonce: Option<i64>,
    pub fee: Option<f64>,
    pub amount: Option<f64>,
    pub source: Option<PendingTxnParty>,
    pub receiver: Option<PendingTxnParty>,
}

impl From<TransactionsQueryTransactions> for PendingTxn {
    fn from(value: TransactionsQueryTransactions) -> Self {
        PendingTxn {
            txn_hash: value.hash,
            kind: value.kind,
            sender_username: value.sender_username,
            nonce: value.nonce,
            fee: value.fee,
            amount: value.amount,
            source: Some(PendingTxnParty {
                public_key: value.from,
            }),
            receiver: Some(PendingTxnParty {
                public_key: value.receiver,
            }),
        }
    }
}
