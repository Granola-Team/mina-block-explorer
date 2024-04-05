use leptos::*;
mod account_activity;
mod blocks;
mod broadcast;
mod common;
mod config;
mod footer;
mod header;
mod icons;
mod next_stakes;
mod root;
mod snarks;
mod stakes;
mod summary;
mod tokens;
mod transactions;
mod zk_apps;

use leptos_meta::provide_meta_context;
use root::Root;

fn main() {
    console_error_panic_hook::set_once();
    provide_meta_context();
    leptos::mount_to_body(|| view! { <Root/> })
}
