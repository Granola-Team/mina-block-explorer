use leptos::*;
mod accounts;
mod blocks;
mod broadcast;
mod common;
mod fee_transfers;
mod footer;
mod header;
mod icons;
mod next_stakes;
mod root;
mod snarks;
mod stakes;
mod summary;
mod transactions;

use root::Root;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <Root/> })
}
