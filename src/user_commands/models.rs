use super::graphql::transactions_query;
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
    pub nonce: Option<i64>,
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
            block_height: None,
            failure_reason: None,
            canonical: None,
            amount: if let Some(amount) = txn.amount {
                match amount.parse() {
                    Ok(parsed_num) => Some(parsed_num),
                    Err(_) => None,
                }
            } else {
                None
            },
            fee: if let Some(fee) = txn.fee {
                match fee.parse() {
                    Ok(parsed_num) => Some(parsed_num),
                    Err(_) => None,
                }
            } else {
                None
            },
            kind: txn.kind,
            to: match txn.receiver.clone() {
                Some(receiver) => receiver.public_key.clone(),
                None => None,
            },
            from: match txn.source {
                Some(source) => source.public_key,
                None => None,
            },
            nonce: txn.nonce,
            memo: txn.memo,
            hash: txn.hash,
            block: None,
            receiver: Some(transactions_query::TransactionsQueryTransactionsReceiver {
                public_key: match txn.receiver {
                    Some(receiver) => receiver.public_key,
                    None => None,
                },
            }),
        }
    }
}
