use serde::{Deserialize, Serialize};

use crate::table::TableData;

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
    from: String,
    receiver: Receiver
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct Block {
    dateTime: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct Receiver {
    publicKey: String
}

impl TableData for TransactionsResponse {
    fn get_columns(&self) -> Vec<String> {
        vec![
                String::from("Date"),
                String::from("From"),
                String::from("To"),
                String::from("Hash"),
                String::from("Fee"),
                String::from("Amount")
            ]
    }

    fn get_rows(&self) -> Vec<Vec<String>> {
        let mut rows = Vec::new();
        for transaction in &self.data {
            let mut data = Vec::new();
            data.push(transaction.block.dateTime.to_string());
            data.push(transaction.from.to_string());
            data.push(transaction.receiver.publicKey.to_string());
            data.push(transaction.fee.to_string());
            data.push(transaction.hash.to_string());
            data.push(transaction.amount.to_string());
            rows.push(data);
        }
        rows
    }
}


