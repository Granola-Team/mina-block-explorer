use crate::{
    common::{components::*, constants::BERKELEY_CHAIN_ID, functions::*},
    summary::models::*,
};
use leptos::*;

#[component]
pub fn SummaryGrid(
    summary: Option<BlockchainSummary>,
    stat: Option<BlockchainStat>,
) -> impl IntoView {
    view! {
        <section class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-5 auto-rows-min gap-4 p-4 pt-0">
            <h2 class="h-0 w-0 overflow-hidden absolute">"Summary"</h2>
            <SummaryItem
                id="epoch"
                label="Epoch"
                value=format_number(
                    summary
                        .as_ref()
                        .and_then(|s| s.chain.as_ref())
                        .and_then(|c| c.get(BERKELEY_CHAIN_ID))
                        .map(|c| c.latest_epoch.to_string())
                        .unwrap_or_default(),
                )
            />
            <SummaryItem
                id="uniqueBlockProducers"
                label="Unique Producers of last 10000 blocks"
                value=stat
                    .as_ref()
                    .map(|s| format_number(s.num_unique_block_producers_last_n_blocks.to_string()))
            />

            <SummaryItem
                id="globalSlot"
                label="Global Slot"
                value=summary.as_ref().map(|s| format_number(s.global_slot.to_string()))
            />
            <SummaryItem
                id="blockchainLength"
                label="Blockchain Length"
                value=summary.as_ref().map(|s| format_number(s.blockchain_length.to_string()))
            />
            <SummaryItem
                id="totalMina"
                label="Total MINA"
                value=summary
                    .as_ref()
                    .map(|s| {
                        format_mina(s.tot_currency().to_string())
                            .split('.')
                            .collect::<Vec<_>>()[0]
                            .to_string()
                    })
            />

            <SummaryItem
                id="circulatingSupply"
                label="Circulating Supply"
                value=summary
                    .as_ref()
                    .map(|s| {
                        format_mina(s.circ_supply().to_string())
                            .split('.')
                            .collect::<Vec<_>>()[0]
                            .to_string()
                    })
            />

            <SummaryItem
                id="totalNumBlocks"
                label="Total Blocks"
                value=summary.as_ref().map(|s| format_number(s.total_num_blocks.to_string()))
            />
            <SummaryItem
                id="totalUserCommands"
                label="Total User Commands"
                value=summary.as_ref().map(|s| format_number(s.total_num_user_commands.to_string()))
            />
            <SummaryItem
                id="totalInternalCommands"
                label="Total Internal Commands"
                value=summary
                    .as_ref()
                    .map(|s| format_number(s.total_num_internal_commands.to_string()))
            />
            <SummaryItem
                id="totalSnarks"
                label="Total SNARKs"
                value=summary.as_ref().map(|s| format_number(s.total_num_snarks.to_string()))
            />
        </section>
    }
}
