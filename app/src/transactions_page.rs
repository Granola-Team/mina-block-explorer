use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct TransactionsResponse {
    data: Vec<Transaction>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Transaction {
    hash: String,
    amount: u64,
    block: Block,
    fee: u64,
    from: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct Block {
    dateTime: String
}

