use leptos::*;
mod account_dialog;
mod addresses;
mod blocks;
mod broadcast;
mod common;
mod footer;
mod header;
mod icons;
mod next_stakes;
mod root;
mod snarks;
mod stakes;
mod summary;
mod transactions;

use leptos_meta::provide_meta_context;
use root::Root;

fn main() {
    console_error_panic_hook::set_once();
    provide_meta_context();
    leptos::mount_to_body(|| view! { <Root/> })
}
