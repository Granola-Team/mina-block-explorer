use leptos::*;

use super::functions::*;
use crate::common::table::*;
use crate::common::functions::*;
use crate::common::components::*;
use crate::common::models::*;


#[component]
pub fn BlockSpotlightFeeTransfersTable(block_state_hash: Option<String>) -> impl IntoView {
    let (bsh_signal, _set_bsh) = create_signal(block_state_hash);
    let resource = create_resource(
        move || bsh_signal.get(),
        move |block_state_hash_opt| async move { load_data(50, block_state_hash_opt).await },
    );

    let records_per_page = 5;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! {
                {
                    match data.feetransfers.len() {
                        0 => view! { <EmptyTable message="No fee transfers related to this block".to_string() /> },
                        _ => {
                            let feetransfers = data.feetransfers;
                            let total_records = feetransfers.len();
                            let ranges = get_ranges(total_records, records_per_page);
                            let range = ranges[current_page.get()-1];
                            let feetransfers_subset = &feetransfers[range[0]..range[1]];
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
                            view! {
                                <Table data=feetransfers_subset pagination=pag/>
                            }
                        }
                    }
                }
            },
            _ => view! { <NullView /> }
        }}
    }
}
