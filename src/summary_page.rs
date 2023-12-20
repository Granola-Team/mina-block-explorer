use std::collections::HashMap;

use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::api_models::{MyError};
use crate::summary_item::{SummaryItem, SummaryItemKind};
use crate::latest_block_page::{LatestBlocksResponse,load_data as load_latest_blocks};
use crate::table::TableData;
use crate::{table::{Table}, table_section::TableSection};

struct OverrideLatestBlockResponse(LatestBlocksResponse);

impl TableData for OverrideLatestBlockResponse {
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
        for block in &self.0.blocks {
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
        linkcols.insert(2, "/summary/accounts/:token".to_owned());
        linkcols.insert(8, "/summary/accounts/:token".to_owned());
        linkcols
    }
}




#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainSummary {
    pub blockchain_length: u64,
    pub circulating_supply: String,
    pub epoch: u16,
    pub slot: u16,
    pub total_currency: String,
}

impl BlockchainSummary {
    fn circ_supply(&self) -> f64 {
        self.circulating_supply.trim().parse().expect("Cannot parse circulating_supply")
    }
    fn tot_currency(&self) -> f64 {
        self.total_currency.trim().parse().expect("Cannot parse total_currency")
    }
}

#[test]
fn test_parsing_floats(){
    let bs = BlockchainSummary {
        circulating_supply: "2345345.4312431243".to_owned(),
        blockchain_length: 314394,
        epoch: 67,
        slot: 4194,
        total_currency: "1105297372.840039233".to_owned(),
    };
    assert_eq!(bs.circ_supply(), 2345345.4312431243);
    assert_eq!(bs.tot_currency(), 1_105_297_372.840_039_3)
}


async fn load_data() -> Result<BlockchainSummary, MyError> {
    let response = reqwest::get("https://api.minaexplorer.com/summary")
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        let summary = response
            .json::<BlockchainSummary>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?;
        Ok(summary)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}

#[component]
pub fn SummaryPage() -> impl IntoView {
    let blockchain_summary_resource = 
        create_resource(|| (), |_| async move { load_data().await });

    let latest_blocks_resource = 
        create_resource(|| (), |_| async move { load_latest_blocks().await });


    view! {
        {move || match blockchain_summary_resource.get() {
            None => view! {
                <div>"Loading..." </div>
            }.into_view(),
            Some(Ok(summary)) => view! { <SummaryGrid summary=summary /> },
            Some(Err(my_error)) => view! {
                <div> { format!("Error: {:#?}", my_error)}</div>
            }.into_view()
        }}
        {move || match latest_blocks_resource.get() {
            None => view! {
                <div>"Loading..." </div>
            }.into_view(),
            Some(Ok(data)) => view! { 
                <TableSection section_heading="Latest Blocks".to_owned()>
                    <Table data=OverrideLatestBlockResponse(data)/>           
                </TableSection>
                <Outlet />
            }.into_view(),
            Some(Err(my_error)) => view! {
                <div> { format!("Error: {:#?}", my_error)}</div>
            }.into_view()
        }}
    }
}

#[component]
fn SummaryGrid(summary: BlockchainSummary) -> impl IntoView {
    view! {        
        <section class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 md:col-start-2 auto-rows-min gap-4 py-4 pt-0">
            <h1 class="h-0 w-0 overflow-hidden absolute">Summary</h1>
            <SummaryItem imgsrc="/assets/img/blockchain_length.svg".to_string() id="blockchainLength".to_string() label="Height".to_string() value={SummaryItemKind::Int64(summary.blockchain_length)} />
            <SummaryItem imgsrc="/assets/img/circulating_supply.svg".to_string() id="circulatingSupply".to_string() label="Circulating Supply".to_string() value={SummaryItemKind::Float64(summary.circ_supply())} />
            <SummaryItem imgsrc="/assets/img/circulating_supply.svg".to_string() id="epoch".to_string() label="Epoch".to_string() value={SummaryItemKind::Int16(summary.epoch)} />
            <SummaryItem imgsrc="/assets/img/circulating_supply.svg".to_string() id="slot".to_string() label="Slot".to_string() value={SummaryItemKind::Int16(summary.slot)} />
            <SummaryItem imgsrc="/assets/img/total_currency.svg".to_string() id="totalCurrency".to_string() label="Total Currency".to_string() value={SummaryItemKind::Float64(summary.tot_currency())} />
        </section>
    }
}
