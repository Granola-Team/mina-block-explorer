use super::functions::load_data;
use leptos::*;
use leptos_router::use_params_map;

#[component]
pub fn TokenHolderPage() -> impl IntoView {
    let memo_params_map = use_params_map();

    let _resource = create_resource(
        move || {
            memo_params_map
                .get()
                .get("id")
                .cloned()
                .expect("/tokens/:id is required ")
        },
        move |id| async move { load_data(id.to_string()).await },
    );

    view! {}
}
