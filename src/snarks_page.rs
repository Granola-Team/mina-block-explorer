
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{table::{TableData, Table}, api_models::MyError};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct SnarksResponse {
    data: Data
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Data {
    snarks: Vec<Snark>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Snark {
    work_ids: Vec<u64>,
    block: Block,
    block_height: u64,
    date_time: String,
    fee: u64,
    prover: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Block {
    state_hash: String
}

impl TableData for SnarksResponse {
    fn get_columns(&self) -> Vec<String> {
        vec![
            String::from("Height"),
            String::from("Date"),
            String::from("Prover"),
            String::from("Work Ids"),
            String::from("State Hash"),
            String::from("Fee"),
        ]
    }

    fn get_rows(&self) -> Vec<Vec<String>> {
        let mut rows = Vec::new();
        for snark in &self.data.snarks {
            let data = vec![
                snark.block_height.to_string(),
                snark.date_time.to_string(),
                snark.prover.to_string(),
                snark.work_ids.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(", "),
                snark.block.state_hash.to_string(),
                snark.fee.to_string()
            ];
            rows.push(data);
        }
        rows
    }
}


async fn load_data() -> Result<SnarksResponse, MyError> {
    let client = reqwest::Client::new();
    let response = client.post("https://graphql.minaexplorer.com")
        .body(r#"{"query":"query MyQuery {\n  snarks(sortBy: BLOCKHEIGHT_DESC, limit: 25) {\n    blockHeight\n    dateTime\n    prover\n    workIds\n    block {\n      stateHash\n    }\n    fee\n  }\n}\n","variables":null,"operationName":"MyQuery"}"#)
        .send()
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        let summary = response
            .json::<SnarksResponse>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?;
        Ok(summary)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}

#[component]
pub fn SnarksPage() -> impl IntoView {
    let resource = create_resource(|| (), |_| async move { load_data().await });

    view! {
        <h1>"SNARKs"</h1>
        <section>
        {move || match resource.get() {
            None => view! {
                <div>"Loading..." </div>
            }.into_view(),
            Some(Ok(data)) => view! { <Table data=data/> },
            Some(Err(my_error)) => view! {
                <div> { format!("Error: {:#?}", my_error)}</div>
            }.into_view()
        }}
        </section>
    }
}