use leptos::*;
mod account_dialog;
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
    leptos::mount_to_body(|| view! { <Root/> })
}
