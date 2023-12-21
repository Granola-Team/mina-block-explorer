use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{
    api_models::MyError,
    table::{Table, TableData}, table_section::TableSection,
};

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

pub async fn load_data(limit: i32, public_key: Option<String>) -> Result<TransactionsResponse, MyError> {
    let mut query = String::from(r#"{"query":"query MyQuery {\n  transactions(limit: ::limit::, sortBy: DATETIME_DESC::query::) {\n    amount\n    fee\n    to\n    from\n    hash\n    block {\n      dateTime\n    }\n    receiver {\n      publicKey\n    }\n  }\n}\n","variables":null,"operationName":"MyQuery"}"#);
    query = query.replace("::limit::", &limit.to_string());
    if let Some(key) = public_key {
        let substring_string = ", query: {from: \\\"::public_key::\\\", canonical: true}".replace("::public_key::", &key);
        query = query.replace("::query::", &substring_string);
    } else {
        query = query.replace("::query::", ", query: {canonical: true}");
    }
    
    let client = reqwest::Client::new();
    let response = client.post("https://graphql.minaexplorer.com")
        .body(query)
        .send()
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        let summary = response
            .json::<TransactionsResponse>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?;
        Ok(summary)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}

#[component]
pub fn TransactionsPage() -> impl IntoView {
    let resource = create_resource(|| (), |_| async move { 
        let limit = 10;
        load_data(limit, None).await 
    });

    view! {
        {move || match resource.get() {
            None => view! {
                <div>"Loading..." </div>
            }.into_view(),
            Some(Ok(data)) => view! { 
                <TableSection section_heading="Transactions".to_owned()>
                    <Table data=data/>
                </TableSection>
             },
            Some(Err(my_error)) => view! {
                <div> { format!("Error: {:#?}", my_error)}</div>
            }.into_view()
        }}
    }
}
