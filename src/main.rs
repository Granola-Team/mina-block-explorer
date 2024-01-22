use leptos::*;
mod accounts;
mod blocks;
mod common;
mod footer;
mod header;
mod icons;
mod root;
mod snarks;
mod stakes;
mod summary;
mod transactions;
mod fee_transfers;

use root::Root;

fn main() {
    leptos::mount_to_body(|| view! { <Root/> })
}
