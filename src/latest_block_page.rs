use leptos::*;
use serde::{Deserialize, Serialize};
use crate::{api_models::MyError, table::{TableData, Table, Pagination}, table_section::TableSection};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct LatestBlocksResponse {
    blocks: Vec<Block>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Block {
    block_height: u64,
    date_time: String,
    creator_account: CreatorAccount,
    transactions: Transactions,
    snark_jobs: Vec<SnarkJob>,
    protocol_state: ProtocolState,
    state_hash: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CreatorAccount {
    public_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Transactions {
    coinbase: u64,
    coinbase_receiver_account: CoinbaseReceiverAccount,
    user_commands: Vec<UserCommand>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ProtocolState {
    consensus_state: ConsensusState
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CoinbaseReceiverAccount {
    public_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ConsensusState {
    slot: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct UserCommand {
    hash: String
}
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct SnarkJob {
    date_time: String
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
    // let pg = Pagination {
    //     current_page:1,
    //     records_per_page: 15,
    //     total_records: 30,
    //     next_page: todo!(),
    //     prev_page: todo!(),
    // };

    view! {
        {move || {
            // let pg_inner = pg.clone();
            match resource.get() {
                None => view! {
                    <div>"Loading..." </div>
                }.into_view(),
                Some(Ok(data)) => view! { 
                    <TableSection section_heading="Latest Blocks".to_owned()>
                        <Table data=data/>
                    </TableSection>
                },
                Some(Err(my_error)) => view! {
                    <div> { format!("Error: {:#?}", my_error)}</div>
                }.into_view()
            }
        }}
    }
}
