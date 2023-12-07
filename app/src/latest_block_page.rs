use leptos::*;
use serde::{Deserialize, Serialize};
use crate::{api_models::MyError, table::{TableData, Table}};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct LatestBlocksResponse {
    blocks: Vec<Block>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct Block {
    blockHeight: u64,
    dateTime: String,
    creatorAccount: CreatorAccount,
    transactions: Transactions,
    snarkJobs: Vec<SnarkJob>,
    protocolState: ProtocolState,
    stateHash: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct CreatorAccount {
    publicKey: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct Transactions {
    coinbase: u64,
    coinbaseReceiverAccount: CoinbaseReceiverAccount,
    userCommands: Vec<UserCommand>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct ProtocolState {
    consensusState: ConsensusState
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct CoinbaseReceiverAccount {
    publicKey: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ConsensusState {
    slot: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct UserCommand {
    hash: String
}
#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct SnarkJob {
    dateTime: String
}

impl TableData for LatestBlocksResponse {
    fn get_columns(&self) -> Vec<String> {
        vec![
                String::from("Height"),
                String::from("Date"),
                String::from("Block Producer"),
                String::from("Coinbase"),
                String::from("Transactions"),
                String::from("SNARKs"),
                String::from("Slot"),
                String::from("State Hash"),
                String::from("Coinbase Receiver"),
            ]
    }

    fn get_rows(&self) -> Vec<Vec<String>> {
        let mut rows = Vec::new();
        for block in &self.blocks {
            let mut data = Vec::new();
            data.push(block.blockHeight.to_string());
            data.push(block.dateTime.to_string());
            data.push(block.creatorAccount.publicKey.to_string());
            data.push(block.transactions.coinbase.to_string());
            data.push(block.transactions.userCommands.len().to_string());
            data.push(block.snarkJobs.len().to_string());
            data.push(block.protocolState.consensusState.slot.to_string());
            data.push(block.stateHash.to_string());
            data.push(block.transactions.coinbaseReceiverAccount.publicKey.to_string());
            rows.push(data);
        }
        rows
    }
}


async fn load_data() -> Result<LatestBlocksResponse, MyError> {
    let response = reqwest::get("https://api.minaexplorer.com/blocks?limit=10")
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        let summary = response
            .json::<LatestBlocksResponse>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?;
        Ok(summary)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}

#[component]
pub fn LatestBlocksPage() -> impl IntoView {
    let resource = create_resource(|| (), |_| async move { load_data().await });

    view! {
        <h1>"Latest Blocks"</h1>
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
