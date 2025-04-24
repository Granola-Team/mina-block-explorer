use super::{components::TokenHoldersMoreDetails, functions::load_data};
use leptos::*;
use leptos_router::use_params_map;

#[component]
pub fn TokenHoldersMoreDetailsSubpage() -> impl IntoView {
    let memo_params_map = use_params_map();

    let resource = create_resource(
        move || {
            (
                memo_params_map
                    .get()
                    .get("id")
                    .cloned()
                    .expect("Account is required"),
                memo_params_map
                    .get()
                    .get("token_id")
                    .cloned()
                    .expect("Token ID is required"),
            )
        },
        move |(account, token_id)| async move {
            load_data(account.to_string(), token_id.to_string()).await
        },
    );

    let get_token = move || {
        resource
            .get()
            .and_then(|res| res.ok())
            .and_then(|rd| rd.token_holders.first().cloned().flatten())
    };

    view! {
        {move || match get_token().map(|token| token.account) {
            Some(account) => {
                view! {
                    <TokenHoldersMoreDetails zkapp=account.zkapp permissions=account.permissions />
                }
            }
            None => ().into_view(),
        }}
    }
}
