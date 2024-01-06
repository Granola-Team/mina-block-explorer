use leptos::*;
mod accounts;
mod blocks;
mod common;
mod footer;
mod header;
mod icons;
mod root;
mod snarks;
mod stakes_page;
mod summary;
mod transactions;

use root::Root;

fn main() {
    leptos::mount_to_body(|| view! { <Root/> })
}
