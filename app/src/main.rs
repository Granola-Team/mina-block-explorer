use leptos::*;
mod root;
mod nav;
mod summary;

use root::Root;

fn main() {
    leptos::mount_to_body(|| view! { <Root/> })
}

