use leptos::*;
use std::collections::HashMap;
use leptos_router::Outlet;
use crate::table::*;
use crate::table_section::*;
use super::functions::*;
use super::graphql::blocks_query::BlocksQueryBlocks;

impl TableData for Vec<Option<BlocksQueryBlocks>> {
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
        // let mut rows = Vec::new();
        // for block in &self.blocks {
        //     let data = vec![
        //         block.block_height.to_string(),
        //         block.date_time.to_string(),
        //         block.creator_account.public_key.to_string(),
        //         block.transactions.coinbase.to_string(),
        //         block.transactions.user_commands.len().to_string(),
        //         block.snark_jobs.len().to_string(),
        //         block.protocol_state.consensus_state.slot.to_string(),
        //         block.state_hash.to_string(),
        //         block.transactions.coinbase_receiver_account.public_key.to_string(),
        //     ];
        //     rows.push(data);
        // }
        // rows
        self.iter()
            .map(|opt_blocks| {
                match opt_blocks {
                    Some(block) => vec![

                    ],
                    None => vec![],
                }
            }).collect::<Vec<_>>()
    }

    fn get_linkable_cols(&self) -> HashMap<i32, String> {
        let mut linkcols: HashMap<i32, String> = HashMap::new();
        linkcols.insert(2, "/blocks/accounts/:token".to_owned());
        linkcols.insert(8, "/blocks/accounts/:token".to_owned());
        linkcols
    }
}

#[component]
pub fn LatestBlocksPage() -> impl IntoView {
    let resource = create_resource(|| (), |_| async move {
        load_data(10, None).await 
    });

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! { 
                <TableSection section_heading="Latest Blocks".to_owned()>
                    <Table data=data.blocks/>           
                </TableSection>
                <Outlet />
            }.into_view(),
            _ => view! { <span /> }.into_view()
        }}
    }
}
