use leptos::*;
mod api_models;
mod root;
mod summary_item;
mod summary_page;
mod latest_block_page;
mod transactions;
mod table;
mod snarks_page;
mod stakes_page;
mod table_section;
mod accounts;
mod header;
mod footer;
mod icons;
mod styles;

use root::Root;

fn main() {
    leptos::mount_to_body(|| view! { <Root/> })
}
