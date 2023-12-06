use leptos::*;
use serde::{Deserialize, Serialize};
// use crate::api_models::MyError;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct LatestBlocksResponse {
    blocks: Vec<Blocks>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct Blocks {
    blockHeight: i32,
    dateTime: String,
    creatorAccount: CreatorAccount,
    transactions: Transactions,
    snarkJobs: Vec<SnarkJob>,
    protocolState: ProtocolState
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct CreatorAccount {
    publicKey: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct Transactions {
    coinbase: i32,
    coinbaseReceiverAccount: CoinbaseReceiverAccount,
    userCommands: Vec<UserCommand>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct ProtocolState {
    consensusState: ConsensusState,
    stateHash: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct CoinbaseReceiverAccount {
    publicKey: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ConsensusState {
    slot: i32
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct UserCommand;
#[derive(Debug, Deserialize, Serialize, Clone)]
struct SnarkJob;

// async fn load_data() -> Result<LatestBlocksResponse, MyError> {
//     let response = reqwest::get("https://api.minaexplorer.com/blocks")
//         .await
//         .map_err(|e| MyError::NetworkError(e.to_string()))?;

//     if response.status().is_success() {
//         let summary = response
//             .json::<LatestBlocksResponse>()
//             .await
//             .map_err(|e| MyError::ParseError(e.to_string()))?;
//         Ok(summary)
//     } else {
//         Err(MyError::NetworkError("Failed to fetch data".into()))
//     }
// }

#[component]
fn Table(columns: Vec<String>) -> impl IntoView {
     view! {
        <table>
            <tr>
                {columns.into_iter()
                    .map(|s| view! { <th>{s}</th>})
                    .collect::<Vec<_>>()}
            </tr>
        </table>
     }
}

#[component]
pub fn TableWrapper() -> impl IntoView {
    let mut cols = Vec::new();
    cols.push(String::from("one"));
    cols.push(String::from("two"));
    cols.push(String::from("three"));
    view! {
        <Table columns=cols/>
    }
}

// #[component]
// pub fn SummaryPage() -> impl IntoView {
//     let resource =
//         create_resource(|| (), |_| async move { load_data().await });

//     view! {
//         <h1>Summary</h1>
//         {move || match blockchain_summary_resource.get() {
//             None => view! {
//                 <div>"Loading..." </div>
//             }.into_view(),
//             Some(Ok(summary)) => view! { <SummaryGrid summary=summary /> },
//             Some(Err(my_error)) => view! {
//                 <div> { format!("Error: {:#?}", my_error)}</div>
//             }.into_view()
//         }}
//     }
// }

// #[component]
// fn LatestBlocks(summary: BlockchainSummary) -> impl IntoView {
//     view! {
//         <section class="grid grid-cols-2 gap-1">
//             <SummaryItem id="blockchainLength".to_string() label="Height".to_string() value={SummaryItemKind::Int64Value(summary.blockchainLength)} />
//             <SummaryItem id="circulatingSupply".to_string() label="Circulating Supply".to_string() value={SummaryItemKind::StrValue(summary.circulatingSupply)} />
//             <SummaryItem id="epoch".to_string() label="Epoch".to_string() value={SummaryItemKind::Int16Value(summary.epoch)} />
//             <SummaryItem id="slot".to_string() label="Slot".to_string() value={SummaryItemKind::Int16Value(summary.slot)} />
//             <SummaryItem id="totalCurrency".to_string() label="Total Currency".to_string() value={SummaryItemKind::StrValue(summary.totalCurrency)} />
//         </section>
//     }
// }

