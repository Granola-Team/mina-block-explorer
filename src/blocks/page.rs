use super::functions::*;
use super::components::*;
use leptos::*;
use leptos_router::*;
use crate::common::components::*;
use crate::common::functions::*;
use crate::common::functions::print_time_since;
use crate::common::spotlight::SpotlightPillVariant;
use crate::common::spotlight::{Spotlight, SpotlightEntry};
use crate::icons::*;

#[component]
pub fn LatestBlocksPage() -> impl IntoView {
    view! { <BlocksSection /> }
}

#[component]
pub fn BlockSpotlight() -> impl IntoView {
    let memo_params_map = use_params_map();
    let resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            let state_hash = value.get("id");
            load_data(10, None, state_hash.cloned()).await
        },
    );

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => {
                let blocks = data.blocks.clone();
                match blocks.get(0).cloned() {
                    Some(Some(block)) => {
                        let state_hash = get_state_hash(&block);
                        let date_time = get_date_time(&block);
                        let summary_items = vec![
                            SpotlightEntry { label: "State Hash".to_string(), value: state_hash, pill_variant: None},
                            SpotlightEntry { label: "Previous State Hash".to_string(), value: get_previous_state_hash(&block), pill_variant: None},
                            SpotlightEntry { label: "Staged Ledger Hash".to_string(), value: get_staged_ledger_hash(&block), pill_variant: None},
                            SpotlightEntry { label: "Snarked Ledger Hash".to_string(), value: get_snarked_ledger_hash(&block), pill_variant: None},
                            SpotlightEntry { label: "Coinbase".to_string(), value: get_coinbase(&block), pill_variant: None},
                            SpotlightEntry { label: "Coinbase Receiver".to_string(), value: get_coinbase_receiver(&block), pill_variant: None},
                            SpotlightEntry { label: "Winning Account".to_string(), value: get_winning_account(&block), pill_variant: None},
                            SpotlightEntry { label: "SNARK Fees".to_string(), value: get_snark_fees(&block), pill_variant: None},
                            SpotlightEntry { label: "Global Slot".to_string(), value: get_global_slot(&block), pill_variant: Some(SpotlightPillVariant::Blue)},
                            SpotlightEntry { label: "Slot".to_string(), value: get_slot(&block), pill_variant: Some(SpotlightPillVariant::Green)},
                            SpotlightEntry { label: "Epoch".to_string(), value: get_epoch(&block), pill_variant: None},
                            SpotlightEntry { label: "Transaction Fees".to_string(), value: get_transaction_fees(&block), pill_variant: None},
                            SpotlightEntry { label: "Blockchain Length".to_string(), value: get_block_height(&block), pill_variant: None},
                            SpotlightEntry { label: "Total Currency".to_string(), value: get_total_currency(&block), pill_variant: None},
                        ];
                        view!{
                            <section class="@container md:col-start-2 md:col-end-3 md:rounded-lg bg-table-section p-0 md:p-4 mb-2">
                                <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">"Block Spotlight"</h1>
                                <Spotlight summary_items=summary_items id=get_state_hash(&block) meta=format!("{} ({})", date_time, print_time_since(&date_time)) >
                                    <BlockIcon width=40/>
                                </Spotlight>
                            </section>
                            <TableSection section_heading="User Commands".to_string()>
                                {
                                    match get_user_commands(&block) {
                                        Some(user_commands) => {
                                            let current_page = 1;
                                            let records_per_page = 10;
                                            let total_records = user_commands.len();
                                            let ranges = get_ranges(total_records, records_per_page);
                                            let range = ranges[current_page-1];
                                            let user_commands_subset = &user_commands[range[0]..range[1]];
                                            let pag = Pagination {
                                                current_page,
                                                records_per_page,
                                                total_records,
                                                next_page: noop,
                                                prev_page: noop,
                                            };
                                            view! { <Table data=user_commands_subset pagination=pag/> }
                                        },
                                        None => view! { <NullView /> }
                                    }
                                }
                            </TableSection>
                        }.into_view()
                    },
                    _ => view! { <NullView /> },
                }
            }
            
            Some(Err(errors)) => view! { <ErrorView err=errors/> },
            _ => view! { <NullView /> }
        }}
        
    }
}