use serde::{Deserialize, Serialize};

use crate::table::TableData;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransactionsResponse {
    pub data: Data,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Data {
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Transaction {
    pub hash: String,
    pub amount: u64,
    pub block: Block,
    pub fee: u64,
    pub from: String,
    pub receiver: Receiver,
    pub to: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub date_time: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Receiver {
    pub public_key: String,
}

impl TableData for TransactionsResponse {
    fn get_columns(&self) -> Vec<String> {
        vec![
            String::from("Date"),
            String::from("From"),
            String::from("To"),
            String::from("Hash"),
            String::from("Fee"),
            String::from("Amount"),
        ]
    }

    fn get_rows(&self) -> Vec<Vec<String>> {
        let mut rows = Vec::new();
        for transaction in &self.data.transactions {
            let data = vec![
                transaction.block.date_time.to_string(),
                transaction.from.to_string(),
                transaction.receiver.public_key.to_string(),
                transaction.fee.to_string(),
                transaction.hash.to_string(),
                transaction.amount.to_string(),
            ];
            rows.push(data);
        }
        rows
    }
}
pub enum Status {
    Pending,
    Complete,
    Unknown
}