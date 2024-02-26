use super::components::*;
use super::models::*;

use crate::common::components::*;

use crate::common::models::*;
use crate::common::search::*;

use leptos::*;
use leptos_router::*;

use super::functions::*;

#[component]
pub fn LatestBlocksPage() -> impl IntoView {
    view! {
        <SearchBar placeholder="Exact search for block hash".to_string()/>
        <PageContainer>
            <BlocksSection/>
        </PageContainer>
    }
}
#[component]
pub fn BlockSpotlightTab() -> impl IntoView {
    view! { <BlockTabContainer content=BlockContent::Spotlight/> }
}

#[component]
pub fn BlockUserCommandsTab() -> impl IntoView {
    view! { <BlockTabContainer content=BlockContent::UserCommands/> }
}

#[component]
pub fn BlockSnarkJobsTab() -> impl IntoView {
    view! { <BlockTabContainer content=BlockContent::SNARKJobs/> }
}

#[component]
pub fn BlockFeeTransfersTab() -> impl IntoView {
    view! { <BlockTabContainer content=BlockContent::FeeTransfers/> }
}

#[component]
pub fn BlockTabbedPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let id = move || memo_params_map.with(|p| p.get("id").cloned().unwrap_or_default());
    let resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            let state_hash = value.get("id");
            load_data(50, None, state_hash.cloned(), None).await
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
                ..Default::default()
            },
            NavEntry {
                href: format!("/blocks/{}/snark-jobs", id()),
                text: "SNARK Jobs".to_string(),
                icon: NavIcon::SNARKs,
                sub_entries: None,
                disabled: false,
                ..Default::default()
            },
            NavEntry {
                href: format!("/blocks/{}/fee-transfers", id()),
                text: "Fee Transfers".to_string(),
                icon: NavIcon::FeeTransfers,
                sub_entries: None,
                disabled: false,
                ..Default::default()
            },
        ]
    };
    move || view! { <TabbedPage tabs=tabs()/> }
}
