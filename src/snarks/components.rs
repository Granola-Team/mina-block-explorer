use leptos::*;

use super::functions::load_data;
use super::functions::*;
use super::graphql::snarks_query::SnarksQuerySnarks;
use crate::accounts::components::*;
use crate::common::components::*;
use crate::common::functions::*;
use crate::common::table::*;
use crate::icons::*;

#[component]
pub fn AccountDialogSnarkJobSection(public_key: Option<String>) -> impl IntoView {
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
                <AccountDialogSectionContainer title=String::from("SNARK Jobs") showing_message={format!("Showing latest {} SNARK jobs", data.snarks.len())} >
                    {
                        match data.snarks.len() {
                            0 => view! { <EmptyTable message="This public key has not completed any SNARK work".to_string() /> },
                            _ => view! {
                                {data.snarks.into_iter()
                                    .map(|opt_snark| {
                                        match opt_snark {
                                            Some(snark) => {
                                                let moments_ago = print_time_since(&get_snark_date_time(&snark));
                                                let date_time = get_snark_date_time(&snark);
                                                let status = get_status(&date_time);
                                                view! {
                                                    <AccountDialogSectionEntryHeader
                                                        status=status
                                                        date=date_time
                                                        moments_ago=moments_ago/>
                                                    <AccountDialogSnarkJobEntry snark=snark/>
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
fn AccountDialogSnarkJobEntry(snark: SnarksQuerySnarks) -> impl IntoView {
    let sub_entries = vec![
        SubEntry {
            label: String::from("Hash"),
            value: snark.block.map_or_else(String::new, |b| {
                b.state_hash.map_or_else(String::new, |sh| sh.to_string())
            }),
        },
        SubEntry {
            label: String::from("Fees Earned"),
            value: snark.fee.map_or_else(String::new, |o| o.to_string()),
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
pub fn AccountOverviewSnarkJobTable(public_key: Option<String>) -> impl IntoView {
    let pk = public_key.clone();
    let resource = create_resource(
        || (),
        move |_| {
            let public_key_inner = public_key.clone();
            async move { load_data(50, public_key_inner, None).await }
        },
    );

    let (href, _set_href) = create_signal(
        pk.as_ref()
            .map(|pk| format!("/snarks?account={}", pk))
            .unwrap_or_else(|| "/snarks".to_string()),
    );

    let records_per_page = 5;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! {
                {
                    match data.snarks.len() {
                        0 => view! { <EmptyTable message="This public key has not completed any SNARK work".to_string() /> },
                        _ => {
                            let pag = build_pagination(data.snarks.len(), records_per_page, current_page.get(), set_current_page);
                            let subset = get_subset(&data.snarks, records_per_page, current_page.get()-1);
                            view! {
                                <Table data=subset pagination=pag />
                                <TableLink href=href.get() text="See all snark jobs".to_string()>
                                    <SnarkIcon />
                                </TableLink>
                            }.into_view()
                        }
                    }
                }
            },
            _ => view! { <span /> }.into_view(),
        }}

    }
}

#[component]
pub fn BlockSpotlightSnarkJobTable(block_state_hash: Option<String>) -> impl IntoView {
    let (bsh_signal, _set_bsh) = create_signal(block_state_hash);
    let resource = create_resource(
        move || bsh_signal.get(),
        move |block_state_hash_opt| async move { load_data(50, None, block_state_hash_opt).await },
    );

    let records_per_page = 5;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! {
                {
                    match data.snarks.len() {
                        0 => view! { <EmptyTable message="No SNARK work related to this block".to_string() /> },
                        _ => {
                            let pag = build_pagination(data.snarks.len(), records_per_page, current_page.get(), set_current_page);
                            let subset = get_subset(&data.snarks, records_per_page, current_page.get()-1);
                            view! {
                                <Table data=subset pagination=pag/>
                            }
                        }
                    }
                }
            },
            _ => view! { <NullView /> }
        }}
    }
}
