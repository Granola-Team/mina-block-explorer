use super::components::*;
use super::functions::*;
use crate::common::components::*;
use crate::common::functions::*;
use crate::common::models::*;
use crate::common::spotlight::*;
use crate::common::search::*;
use crate::icons::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn LatestBlocksPage() -> impl IntoView {
    view! {
        <PageContainer>
            <BlocksSection />
        </PageContainer>
    }
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
    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        <SearchBar />
        <PageContainer>
            {move || match resource.get() {
                Some(Ok(data)) => {
                    let blocks = data.blocks.clone();
                    match blocks.first().cloned() {
                        Some(Some(block)) => {
                            let state_hash = get_state_hash(&block);
                            let date_time = get_date_time(&block);
                            let spotlight_items = vec![
                                SpotlightEntry { label: "State Hash".to_string(), value: state_hash, pill_variant: None},
                                SpotlightEntry { label: "Previous State Hash".to_string(), value: get_previous_state_hash(&block), pill_variant: None},
                                SpotlightEntry { label: "Staged Ledger Hash".to_string(), value: get_staged_ledger_hash(&block), pill_variant: None},
                                SpotlightEntry { label: "Snarked Ledger Hash".to_string(), value: get_snarked_ledger_hash(&block), pill_variant: None},
                                SpotlightEntry { label: "Coinbase".to_string(), value: get_coinbase(&block), pill_variant: None},
                                SpotlightEntry { label: "Coinbase Receiver".to_string(), value: get_coinbase_receiver(&block), pill_variant: None},
                                SpotlightEntry { label: "Winning Account".to_string(), value: get_winning_account(&block), pill_variant: None},
                                SpotlightEntry { label: "SNARK Fees".to_string(), value: get_snark_fees(&block), pill_variant: None},
                                SpotlightEntry { label: "Global Slot".to_string(), value: get_global_slot(&block), pill_variant: Some(PillVariant::Blue)},
                                SpotlightEntry { label: "Slot".to_string(), value: get_slot(&block), pill_variant: Some(PillVariant::Green)},
                                SpotlightEntry { label: "Epoch".to_string(), value: get_epoch(&block), pill_variant: None},
                                SpotlightEntry { label: "Transaction Fees".to_string(), value: get_transaction_fees(&block), pill_variant: None},
                                SpotlightEntry { label: "Blockchain Length".to_string(), value: get_block_height(&block), pill_variant: None},
                                SpotlightEntry { label: "Total Currency".to_string(), value: get_total_currency(&block), pill_variant: None},
                            ];
                            view!{
                                <SpotlightSection header="Block Spotlight".to_string()
                                    spotlight_items=spotlight_items
                                    id=get_state_hash(&block)
                                    meta=format!("{} ({})", date_time, print_time_since(&date_time)) >
                                    <BlockIcon width=40/>
                                </SpotlightSection>
                                <TableSection section_heading="User Commands".to_string()>
                                    {
                                        move || match get_user_commands(&block) {
                                            Some(user_commands) => {
                                                let total_records = user_commands.len();
                                                let ranges = get_ranges(total_records, records_per_page);
                                                let range = ranges[current_page.get()-1];
                                                let user_commands_subset = &user_commands[range[0]..range[1]];
                                                let pag = Pagination {
                                                    current_page: current_page.get(),
                                                    records_per_page,
                                                    total_records,
                                                    next_page: Callback::from(move |_| {
                                                        let set_current_page_inner = set_current_page;
                                                        set_current_page_inner.update(|cp| *cp += 1);
                                                    }),
                                                    prev_page: Callback::from(move |_| {
                                                        let set_current_page_inner = set_current_page;
                                                        set_current_page_inner.update(|cp| *cp -= 1);
                                                    }),
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
        </PageContainer>
    }
}
