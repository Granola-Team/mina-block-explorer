use super::{functions::*, graphql::snarks_query::SnarksQuerySnarks};
use crate::{
    account_activity::components::*,
    common::{functions::*, table::*},
};
use leptos::*;

#[component]
pub fn AccountDialogSnarkJobSection(snarks: Vec<Option<SnarksQuerySnarks>>) -> impl IntoView {
    let snarks_inner = snarks.clone();
    let has_snarks = move || !snarks.clone().is_empty();

    view! {
        <AccountDialogSectionContainer
            title=String::from("SNARK Jobs")
            showing_message=format!("{} most recent SNARK jobs", snarks_inner.len())
        >

            <Show
                when=has_snarks
                fallback=move || {
                    view! {
                        <EmptyTable message="This public key has not completed any SNARK work" />
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
                                fallback=move || ().into_view()
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
                                        <AccountDialogSnarkJobEntry snark=snark.clone() />
                                        <AccountDialogEntryDivider />
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
                .map(|se| {
                    view! {
                        <AccountDialogSubsectionRow
                            label=se.label
                            el=convert_to_ellipsis(se.value)
                        />
                    }
                })
                .collect::<Vec<_>>()}
        </AccountDialogSubsectionTable>
    }
    .into_view()
}
