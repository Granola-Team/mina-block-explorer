use super::{components::*, functions::*, models::*};
use crate::{
    blocks::graphql::blocks_query,
    common::{components::*, constants::*, models::*},
};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;
use leptos_use::{storage::*, use_interval, UseIntervalReturn};

#[component]
pub fn BlockSpotlightTab() -> impl IntoView {
    view! {
        <Title text="Block Overview | Spotlight" />
        <BlockTabContainer content=BlockContent::Spotlight />
    }
}

#[component]
pub fn BlockUserCommandsTab() -> impl IntoView {
    view! {
        <Title text="Block Overview | User Commands" />
        <BlockTabContainer content=BlockContent::UserCommands />
    }
}

#[component]
pub fn BlockSnarkJobsTab() -> impl IntoView {
    view! {
        <Title text="Block Overview | SNARK Jobs" />
        <BlockTabContainer content=BlockContent::SNARKJobs />
    }
}

#[component]
pub fn BlockInternalCommandsTab() -> impl IntoView {
    view! {
        <Title text="Block Overview | Internal Commands" />
        <BlockTabContainer content=BlockContent::FeeTransfers />
    }
}

#[component]
pub fn BlockAnalyticsTab() -> impl IntoView {
    view! {
        <Title text="Block Overview | Analytics" />
        <BlockTabContainer content=BlockContent::Analytics />
    }
}

#[component]
pub fn BlocksLocalStorage() -> impl IntoView {
    let (_, set_blocks, _) =
        use_local_storage::<blocks_query::ResponseData, JsonSerdeCodec>(BLOCKS_STORAGE_KEY);
    let UseIntervalReturn { counter, .. } = use_interval(LIVE_RELOAD_INTERVAL);

    let resource = create_resource(
        move || counter.get(),
        |_| async move { load_data(1000, None, None, None, None, Some(true)).await },
    );

    create_effect(move |_| {
        resource
            .get()
            .and_then(|res| res.ok())
            .map(|data| set_blocks.set(data))
    });

    ().into_view()
}

#[component]
pub fn BlockTabbedPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let id = move || memo_params_map.with(|p| p.get("id").cloned().unwrap_or_default());
    let resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            let state_hash = value.get("id");
            load_data(1, None, state_hash.cloned(), None, None, None).await
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
                ..Default::default()
            },
            NavEntry {
                href: format!("/blocks/{}/commands/user", id()),
                text: "User Commands".to_string(),
                icon: NavIcon::Transactions,
                number_bubble: option_block.get().as_ref().and_then(get_transaction_count),
                ..Default::default()
            },
            NavEntry {
                href: format!("/blocks/{}/snark-jobs", id()),
                text: "SNARK Jobs".to_string(),
                icon: NavIcon::SNARKs,
                number_bubble: option_block.get().as_ref().and_then(get_snark_job_count),
                ..Default::default()
            },
            NavEntry {
                href: format!("/blocks/{}/commands/internal", id()),
                text: "Internal Commands".to_string(),
                icon: NavIcon::FeeTransfers,
                number_bubble: option_block.get().as_ref().and_then(get_fee_transfer_count),
                ..Default::default()
            },
            NavEntry {
                href: format!("/blocks/{}/analytics", id()),
                text: "Analytics".to_string(),
                icon: NavIcon::Analytics,
                ..Default::default()
            },
        ]
    };
    move || view! { <TabbedPage tabs=tabs() /> }
}
