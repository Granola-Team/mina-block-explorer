use leptos::*;
mod root;
mod nav;
mod summary_page;
mod account_page;
mod api_models;
mod summary_item;

use root::Root;

fn main() {
    leptos::mount_to_body(|| view! { <Root/> })
}

