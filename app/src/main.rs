use leptos::*;
mod root;
mod nav;
mod summary;
mod api_models;

use root::Root;

fn main() {
    leptos::mount_to_body(|| view! { <Root/> })
}

