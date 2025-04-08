use super::functions::load_data;
use leptos::*;
use leptos_meta::Title;
use leptos_router::use_params_map;

#[component]
pub fn TokenHolderPage() -> impl IntoView {
    let memo_params_map = use_params_map();

    let resource = create_resource(
        move || {
            memo_params_map
                .get()
                .get("id")
                .cloned()
                .expect("Token ID is required")
        },
        move |id| async move { load_data(id.to_string()).await },
    );

    let get_token = move || {
        resource
            .get()
            .and_then(|res| res.ok())
            .and_then(|rd| rd.token_holders.first().cloned().flatten())
    };

    view! {
        <Title
            formatter=move |text| format!("Token Overview | {text}")
            text=move || get_token().map(|token| token.symbol.to_string()).unwrap_or_default()
        />
    }
}
