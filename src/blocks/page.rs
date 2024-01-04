use leptos::*;
use std::collections::HashMap;
use leptos_router::Outlet;
use serde::{Deserialize, Serialize};
use crate::{common::models::MyError, table::{TableData, Table}, table_section::TableSection};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LatestBlocksResponse {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub block_height: u64,
    pub date_time: String,
    pub creator_account: CreatorAccount,
    pub transactions: Transactions,
    pub snark_jobs: Vec<SnarkJob>,
    pub protocol_state: ProtocolState,
    pub state_hash: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatorAccount {
    pub public_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    pub coinbase: u64,
    pub coinbase_receiver_account: CoinbaseReceiverAccount,
    pub user_commands: Vec<UserCommand>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolState {
    pub consensus_state: ConsensusState
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinbaseReceiverAccount {
    pub public_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConsensusState {
    pub slot: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserCommand {
    pub hash: String
}
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SnarkJob {
    pub date_time: String
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
            let data = vec![
                block.block_height.to_string(),
                block.date_time.to_string(),
                block.creator_account.public_key.to_string(),
                block.transactions.coinbase.to_string(),
                block.transactions.user_commands.len().to_string(),
                block.snark_jobs.len().to_string(),
                block.protocol_state.consensus_state.slot.to_string(),
                block.state_hash.to_string(),
                block.transactions.coinbase_receiver_account.public_key.to_string(),
            ];
            rows.push(data);
        }
        rows
    }

    fn get_linkable_cols(&self) -> HashMap<i32, String> {
        let mut linkcols: HashMap<i32, String> = HashMap::new();
        linkcols.insert(2, "/blocks/accounts/:token".to_owned());
        linkcols.insert(8, "/blocks/accounts/:token".to_owned());
        linkcols
    }
}


pub async fn load_data() -> Result<LatestBlocksResponse, MyError> {
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
        {move || match resource.get() {
            Some(Ok(data)) => view! { 
                <TableSection section_heading="Latest Blocks".to_owned()>
                    <Table data=data/>           
                </TableSection>
                <Outlet />
            }.into_view(),
            _ => view! { <span /> }.into_view()
        }}
    }
}
