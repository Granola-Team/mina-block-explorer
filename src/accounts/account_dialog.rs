use leptos::*;
use leptos_router::*;

use crate::common::models::MyError;

use super::components::*;
use super::functions::*;

#[component]
pub fn AccountDialogView() -> impl IntoView {
    let location = use_location();
    let base = get_base_page_path(location);
    let memo_params_map = use_params_map();

    let account_resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            if let Some(id) = value.get("id").cloned() {
                let id_clone = id.clone();
                load_data(&id_clone).await
            } else {
                Err(MyError::ParseError(String::from(
                    "Could not parse id parameter from url",
                )))
            }
        },
    );

    let public_key = move || memo_params_map.with(|p| p.get("id").cloned());

    view! {
        {move || match account_resource.get() {
            Some(Ok(res)) => {
                view! {
                    <AccountDialog
                        public_key=public_key().unwrap_or_default()
                        path_base=base.to_owned()
                        account=Some(res.account)
                    />
                }
            }
            None => {
                view! {
                    <AccountDialog
                        public_key=public_key().unwrap_or_default()
                        path_base=base.to_owned()
                        account=None
                    />
                }
            }
            _ => view! { <span></span> }.into_view(),
        }}
    }
}
