use leptos::*;
mod account_page;
mod api_models;
mod nav;
mod root;
mod summary_item;
mod summary_page;

use root::Root;

fn main() {
    leptos::mount_to_body(|| view! { <Root/> })
}
