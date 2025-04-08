use super::functions::load_data;
use crate::{common::spotlight::SpotlightSection, icons::TokenSymbol};
use leptos::*;
use leptos_meta::Title;
use leptos_router::use_params_map;

#[component]
pub fn TokenHolderPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let (token_symbol_sig, set_token) = create_signal::<String>("".to_string());

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

    create_effect(move |_| {
        if let Some(token) = get_token() {
            set_token.set(token.symbol.to_string());
        }
    });

    view! {
        <Title
            formatter=move |text| format!("Token Overview | {text}")
            text=move || token_symbol_sig.get()
        />
        {move || match get_token() {
            Some(token) => {
                view! {
                    <SpotlightSection
                        header="Token Overview"
                        spotlight_items=vec![]
                        meta=Some(format!("Symbol: {}", token.symbol))
                        id=memo_params_map.get().get("id").cloned()
                    >
                        <TokenSymbol width=40 />
                    </SpotlightSection>
                }
            }
            None => {
                view! {
                    <SpotlightSection
                        header="Token Overview"
                        spotlight_items=vec![]
                        meta=None
                        id=None
                    >
                        <TokenSymbol width=40 />
                    </SpotlightSection>
                }
            }
        }}
    }
}
