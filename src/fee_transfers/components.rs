use super::functions::*;
use crate::common::{components::*, functions::*, table::*};
use charming::{
    component::{Legend, Title},
    series::*,
    Chart, WasmRenderer,
};
use leptos::*;
use std::collections::HashMap;

#[component]
pub fn BlockSpotlightFeeTransferAnalytics(block_state_hash: Option<String>) -> impl IntoView {
    let (bsh_signal, _set_bsh) = create_signal(block_state_hash);
    let resource = create_resource(
        move || bsh_signal.get(),
        move |block_state_hash_opt| async move { load_data(50, block_state_hash_opt).await },
    );

    let (data, set_data) = create_signal(HashMap::new());

    create_effect(move |_| {
        let mut pie_hashmap = HashMap::new();
        if let Some(Ok(data)) = resource.get() {
            data.feetransfers.into_iter().for_each(|row| {
                if let Some(r) = row.clone() {
                    match (r.fee, r.recipient) {
                        (Some(fee), Some(mut recipient)) => {
                            recipient.truncate(12);
                            let recipient = recipient.to_string();
                            if !pie_hashmap.contains_key(&recipient) {
                                pie_hashmap.insert(recipient, fee);
                            } else {
                                if let Some(val) = pie_hashmap.get_mut(&recipient) {
                                    *val += fee;
                                }
                            }
                        }
                        (_, _) => (),
                    }
                }
                logging::log!("{}", "iterating...");
            });

            set_data.set(pie_hashmap);
        }
    });

    let action = create_action(move |input: &HashMap<String, i64>| {
        let input = input.clone();
        async move {
            let mut data = input
                .iter()
                .map(|(key, val)| (*val, key))
                .collect::<Vec<_>>();

            // Sort the vector in descending order
            data.sort_by(|a, b| b.cmp(a));

            // Split the vector into two parts
            let size = data.len();
            let (top_five, rest) = data.split_at_mut(5.min(size));

            // Aggregate the remaining entries
            let binding = String::from("Other");
            let aggregated = rest.iter().fold((0, &binding), |mut acc, tup| {
                acc.0 += tup.0;
                acc
            });

            // Append the aggregated tuple to the top six
            let mut result = top_five.to_vec();
            if !rest.is_empty() {
                result.push(aggregated);
            }

            let series = Pie::new()
                .name("Fee Transfer Recipient")
                .radius(vec!["50", "100"])
                .center(vec!["50%", "50%"])
                .data(result);
            let chart = Chart::new()
                .title(Title::new().text("Top Internal Transfers"))
                .legend(Legend::new().top("bottom"))
                .series(series);
            let renderer = WasmRenderer::new(375, 375);

            renderer.render("chart", &chart).unwrap();
        }
    });

    create_effect(move |_| {
        if data.get().is_empty() {
            return;
        }
        action.dispatch(data.get());
    });

    view! { <div id="chart" class="p-4 md:p-8"></div> }
}

#[component]
pub fn BlockInternalCommandsTable(block_state_hash: Option<String>) -> impl IntoView {
    let (bsh_signal, _set_bsh) = create_signal(block_state_hash);
    let resource = create_resource(
        move || bsh_signal.get(),
        move |block_state_hash_opt| async move { load_data(50, block_state_hash_opt).await },
    );

    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => {
                view! {
                    {match data.feetransfers.len() {
                        0 => {
                            view! {
                                <EmptyTable message="No internal commands for this block"
                                    .to_string()/>
                            }
                        }
                        _ => {
                            let pag = build_pagination(
                                data.feetransfers.len(),
                                records_per_page,
                                current_page.get(),
                                set_current_page,
                            );
                            let subset = get_subset(
                                &data.feetransfers,
                                records_per_page,
                                current_page.get() - 1,
                            );
                            view! { <Table data=subset pagination=pag/> }
                        }
                    }}
                }
            }
            _ => view! { <NullView/> },
        }}
    }
}
