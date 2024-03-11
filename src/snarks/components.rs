use super::{
    functions::{load_data, *},
    graphql::snarks_query::SnarksQuerySnarks,
};
use crate::{
    account_dialog::components::*,
    common::{components::*, functions::*, table::*},
    icons::*,
};
use leptos::*;
use leptos_router::create_query_signal;

#[component]
pub fn AccountDialogSnarkJobSection(snarks: Vec<Option<SnarksQuerySnarks>>) -> impl IntoView {
    let snarks_inner = snarks.clone();
    let has_snarks = move || !snarks.clone().is_empty();

    view! {
        <AccountDialogSectionContainer
            title=String::from("SNARK Jobs")
            showing_message=format!("Showing latest {} SNARK jobs", snarks_inner.len())
        >

            <Show
                when=has_snarks
                fallback=move || {
                    view! {
                        <EmptyTable message="This public key has not completed any SNARK work"
                            .to_string()/>
                    }
                }
            >

                {snarks_inner
                    .iter()
                    .map(|opt_snark| {
                        let check_opt_snark = opt_snark.clone();
                        let snark = opt_snark.clone().unwrap();
                        view! {
                            <Show
                                when=move || check_opt_snark.is_some()
                                fallback=move || view! { <NullView/> }
                            >

                                {
                                    let moments_ago = print_time_since(
                                        &get_snark_date_time(&snark),
                                    );
                                    let date_time = get_snark_date_time(&snark);
                                    let status = get_status(&date_time);
                                    view! {
                                        <AccountDialogSectionEntryHeader
                                            status=status
                                            date=date_time
                                            moments_ago=moments_ago
                                        />
                                        <AccountDialogSnarkJobEntry snark=snark.clone()/>
                                        <AccountDialogEntryDivider/>
                                    }
                                }

                            </Show>
                        }
                            .into_view()
                    })
                    .collect::<Vec<_>>()}

            </Show>
        </AccountDialogSectionContainer>
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
        <AccountDialogSubsectionTable>
            {sub_entries
                .into_iter()
                .map(|se| view! { <AccountDialogSubsectionRow label=se.label value=se.value/> })
                .collect::<Vec<_>>()}
        </AccountDialogSubsectionTable>
    }
    .into_view()
}

#[component]
pub fn AccountOverviewSnarkJobTable(public_key: Option<String>) -> impl IntoView {
    let pk = public_key.clone();
    let (canonical_qs, _) = create_query_signal::<bool>("canonical");
    let resource = create_resource(
        move || canonical_qs.get(),
        move |canonical| {
            let public_key_inner = public_key.clone();
            async move { load_data(50, public_key_inner, None, canonical).await }
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
            Some(Ok(data)) => {
                view! {
                    {match data.snarks.len() {
                        0 => {
                            view! {
                                <EmptyTable message="This public key has not completed any SNARK work"
                                    .to_string()/>
                            }
                        }
                        _ => {
                            let pag = build_pagination(
                                data.snarks.len(),
                                records_per_page,
                                current_page.get(),
                                set_current_page,
                            );
                            let subset = get_subset(
                                &data.snarks,
                                records_per_page,
                                current_page.get() - 1,
                            );
                            view! {
                                <Table data=subset pagination=pag/>
                                <TableLink href=href.get() text="See all snark jobs".to_string()>
                                    <CheckCircleIcon/>
                                </TableLink>
                            }
                                .into_view()
                        }
                    }}
                }
            }
            _ => view! { <span></span> }.into_view(),
        }}
    }
}
