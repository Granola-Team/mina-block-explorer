use leptos::*;
use leptos_router::*;

use super::functions::*;
use super::graphql::blocks_query::BlocksQueryBlocks;
use super::models::*;
use crate::accounts::components::*;
use crate::common::components::EmptyTable;
use crate::common::components::*;
use crate::common::functions::*;
use crate::icons::*;

#[component]
pub fn AccountDialogBlocksSection(public_key: Option<String>) -> impl IntoView {
    let resource = create_resource(
        || (),
        move |_| {
            let public_key_inner = public_key.clone();
            async move { load_data(3, public_key_inner, None).await }
        },
    );

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! {
                <AccountDialogSectionContainer title=String::from("Block Production") showing_message={format!("Showing latest {} blocks", data.blocks.len())} >
                    {
                        match data.blocks.len() {
                            0 => view! { <EmptyTable message="This public key has no block production".to_string() /> },
                            _ => view! {
                                {data.blocks.into_iter()
                                    .map(|opt_block| {
                                        match opt_block {
                                            Some(block) => {
                                                let moments_ago = print_time_since(&get_date_time(&block));
                                                let date_time = get_date_time(&block);
                                                let status = get_status(&date_time);
                                                view! {
                                                    <AccountDialogSectionEntryHeader
                                                        status=status
                                                        date=date_time
                                                        moments_ago=moments_ago/>
                                                    <AccountDialogBlockEntry block=block/>
                                                    <AccountDialogEntryDivider />
                                                }.into_view()
                                            },
                                            None => view! { <span /> }.into_view(),
                                        }
                                    }).collect::<Vec<_>>()}
                            }.into_view()
                        }
                    }
                </AccountDialogSectionContainer>
            },
            _ => view! { <span /> }.into_view(),
        }}

    }
}

struct SubEntry {
    label: String,
    value: String,
}

#[component]
fn AccountDialogBlockEntry(block: BlocksQueryBlocks) -> impl IntoView {
    let sub_entries = vec![
        SubEntry {
            label: String::from("Hash"),
            value: get_state_hash(&block),
        },
        SubEntry {
            label: String::from("Coinbase"),
            value: get_coinbase(&block),
        },
    ];
    view! {
        <div class="w-full flex justify-between">
            {sub_entries.into_iter()
                .map(|se| view! {
                    <AccountDialogSectionSubEntry label=se.label value=se.value />
                })
            .collect::<Vec<_>>()}
        </div>
    }
    .into_view()
}

#[component]
pub fn BlocksSection() -> impl IntoView {
    let query_params_map = use_query_map();

    let resource = create_resource(
        move || query_params_map.get(),
        |value| async move {
            let public_key = value.get("account");
            load_data(10, public_key.cloned(), None).await
        },
    );

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! {
                <TableSection section_heading="Blocks".to_owned()>
                    <Table data=data.blocks/>
                </TableSection>
                <Outlet />
            }.into_view(),
            _ => view! { <span/> }.into_view()
        }}
    }
}

#[component]
pub fn SummaryPageBlocksSection() -> impl IntoView {
    let resource = create_resource(|| (), |_| async move { load_data(10, None, None).await });

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! {
                <TableSection section_heading="Blocks".to_owned()>
                    <Table data=SummaryPageBlocksQueryBlocks(data.blocks)/>
                </TableSection>
                <Outlet />
            }.into_view(),
            _ => view! { <span/> }.into_view()
        }}
    }
}

#[component]
pub fn AccountOverviewBlocksTable(public_key: Option<String>) -> impl IntoView {
    let pk = public_key.clone();
    let resource = create_resource(
        || (),
        move |_| {
            let public_key_inner = public_key.clone();
            async move { load_data(5, public_key_inner, None).await }
        },
    );

    let (href, _set_href) = create_signal(
        pk.as_ref()
            .map(|pk| format!("/blocks?account={}", pk))
            .unwrap_or_else(|| "/blocks".to_string()),
    );

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! {
                {
                    match data.blocks.len() {
                        0 => view! { <EmptyTable message="This public key has no block production".to_string() /> },
                        _ => view! {
                            <Table data=data.blocks />
                            <TableLink href=href.get() text="See all block production".to_string()>
                                <BlockIcon />
                            </TableLink>
                        }.into_view()
                    }
                }
            },
            _ => view! { <span /> }.into_view(),
        }}

    }
}
