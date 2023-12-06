use leptos::*;
use serde::{Deserialize, Serialize};
use crate::api_models::MyError;

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
fn Table(columns: Vec<String>, data: LatestBlocksResponse) -> impl IntoView {
    view! {
       <table>
           <tr>
               {columns.into_iter()
                   .map(|s| view! { <th>{s}</th>})
                   .collect::<Vec<_>>()}
           </tr>
           {data.blocks.into_iter()
            .map(|d| view! { <tr>
                <td>{d.blockHeight}</td>
                <td>{d.dateTime}</td>
                <td>{d.creatorAccount.publicKey}</td>
                <td>{d.transactions.coinbase}</td>
                <td>{d.transactions.userCommands.len()}</td>
                <td>{d.snarkJobs.len()}</td>
                <td>{d.protocolState.consensusState.slot}</td>
                <td>{d.stateHash}</td>
                <td>{d.transactions.coinbaseReceiverAccount.publicKey}</td>
            </tr>})
            .collect::<Vec<_>>()}
       </table>
    }
}

#[component]
pub fn LatestBlocksPage() -> impl IntoView {
    let resource = create_resource(|| (), |_| async move { load_data().await });

    view! {
        <h1>"Latest Blocks"</h1>
        {move || match resource.get() {
            None => view! {
                <div>"Loading..." </div>
            }.into_view(),
            Some(Ok(data)) => view! { <Table columns=vec![
                String::from("Height"),
                String::from("Date"),
                String::from("Block Producer"),
                String::from("Coinbase"),
                String::from("Transactions"),
                String::from("SNARKs"),
                String::from("Slot"),
                String::from("State Hash"),
                String::from("Coinbase Receiver"),
            ] data=data/> },
            Some(Err(my_error)) => view! {
                <div> { format!("Error: {:#?}", my_error)}</div>
            }.into_view()
        }}
    }
}
