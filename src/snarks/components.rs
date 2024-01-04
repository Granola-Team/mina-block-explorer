use leptos::*;

use crate::accounts::components::*;
use super::functions::load_data;
use super::graphql::snarks_query::SnarksQuerySnarks;
use super::functions::*;
use crate::icons::*;
use crate::common::functions::*;

#[component]
pub fn AccountDialogSnarkJobSection(public_key: Option<String>) -> impl IntoView {

    let resource = create_resource(|| (), move |_| {
        let public_key_inner = public_key.clone();
        async move { load_data(3,public_key_inner).await }
    });

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! {
                <AccountDialogSectionContainer title=String::from("SNARK Jobs") showing_message={format!("Showing latest {} SNARK jobs", data.snarks.len())} >
                    {
                        match data.snarks.len() {
                            0 => view! { <NoSnarkJobs /> },
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
    value: String
}

#[component]
fn NoSnarkJobs() -> impl IntoView {
    view! { 
        <div class="flex text-base text-slate-400 items-center justify-center p-8">
            <NoIcon /> 
            <span class="text-sm">"This public key has not completed any SNARK work"</span>
        </div>
    }
}

#[component]
fn AccountDialogSnarkJobEntry(snark: SnarksQuerySnarks) -> impl IntoView {
    let sub_entries = vec![
        SubEntry {
            label: String::from("Hash"),
            value: snark.block.map_or_else(String::new, |b| b.state_hash.map_or_else(String::new, |sh| sh.to_string()))
        },
        SubEntry {
            label: String::from("Fees Earned"),
            value: snark.fee.map_or_else(String::new, |o| o.to_string())
        }
    ];
    view! {
        <div class="w-full flex justify-between">
            {sub_entries.into_iter()
                .map(|se| view! {
                    <AccountDialogSectionSubEntry label=se.label value=se.value />
                })
            .collect::<Vec<_>>()}            
        </div>
    }.into_view()
}