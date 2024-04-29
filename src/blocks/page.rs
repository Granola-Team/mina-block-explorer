use super::{components::*, functions::*, models::*};
use crate::common::{components::*, constants::*, models::*, search::*};

use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

#[component]
pub fn LatestBlocksPage() -> impl IntoView {
    view! {
        <Title text="Blocks | Search for Mina Blocks"/>
        <SearchBar placeholder=MULTI_SEARCH_PLACEHOLDER_TEXT/>
        <PageContainer>
            <BlocksSection/>
        </PageContainer>
    }
}
#[component]
pub fn BlockSpotlightTab() -> impl IntoView {
    view! {
        <Title text="Block Overview | Spotlight"/>
        <BlockTabContainer content=BlockContent::Spotlight/>
    }
}

#[component]
pub fn BlockUserCommandsTab() -> impl IntoView {
    view! {
        <Title text="Block Overview | User Commands"/>
        <BlockTabContainer content=BlockContent::UserCommands/>
    }
}

#[component]
pub fn BlockSnarkJobsTab() -> impl IntoView {
    view! {
        <Title text="Block Overview | SNARK Jobs"/>
        <BlockTabContainer content=BlockContent::SNARKJobs/>
    }
}

#[component]
pub fn BlockInternalCommandsTab() -> impl IntoView {
    view! {
        <Title text="Block Overview | Internal Commands"/>
        <BlockTabContainer content=BlockContent::FeeTransfers/>
    }
}

#[component]
pub fn BlockAnalyticsTab() -> impl IntoView {
    view! {
        <Title text="Block Overview | Analytics"/>
        <BlockTabContainer content=BlockContent::Analytics/>
    }
}

#[component]
pub fn BlockTabbedPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let id = move || memo_params_map.with(|p| p.get("id").cloned().unwrap_or_default());
    let resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            let state_hash = value.get("id");
            load_data(
                TABLE_RECORD_SIZE,
                None,
                state_hash.cloned(),
                None,
                None,
                None,
            )
            .await
        },
    );

    let (option_block, set_option_block) = create_signal(None);

    create_effect(move |_| {
        let option_block = resource
            .get()
            .and_then(|res| res.ok())
            .and_then(|res| res.blocks.first().cloned().unwrap_or_default());

        set_option_block.set(option_block);
    });

    provide_context(option_block);

    let tabs = move || {
        vec![
            NavEntry {
                href: format!("/blocks/{}/spotlight", id()),
                text: "Block Spotlight".to_string(),
                icon: NavIcon::Blocks,
                sub_entries: None,
                disabled: false,
                ..Default::default()
            },
            NavEntry {
                href: format!("/blocks/{}/user-commands", id()),
                text: "User Commands".to_string(),
                icon: NavIcon::Transactions,
                number_bubble: option_block.get().as_ref().and_then(get_transaction_count),
                sub_entries: None,
                disabled: false,
            },
            NavEntry {
                href: format!("/blocks/{}/snark-jobs", id()),
                text: "SNARK Jobs".to_string(),
                icon: NavIcon::SNARKs,
                number_bubble: option_block.get().as_ref().and_then(get_snark_job_count),
                sub_entries: None,
                disabled: false,
            },
            NavEntry {
                href: format!("/blocks/{}/internal-commands", id()),
                text: "Internal Commands".to_string(),
                icon: NavIcon::FeeTransfers,
                number_bubble: option_block
                    .get()
                    .as_ref()
                    .and_then(get_fee_transfer_count)
                    .map(|c| {
                        if option_block
                            .get()
                            .and_then(|block| {
                                block.transactions.and_then(|trx| {
                                    trx.coinbase_receiver_account.and_then(|ra| ra.public_key)
                                })
                            })
                            .is_some()
                        {
                            c + 1
                        } else {
                            c
                        }
                    }),
                sub_entries: None,
                disabled: false,
            },
            NavEntry {
                href: format!("/blocks/{}/analytics", id()),
                text: "Analytics".to_string(),
                icon: NavIcon::Analytics,
                number_bubble: None,
                sub_entries: None,
                disabled: false,
            },
        ]
    };
    move || view! { <TabbedPage tabs=tabs()/> }
}
