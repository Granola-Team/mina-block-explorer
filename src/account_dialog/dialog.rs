use leptos::*;
use leptos_router::*;

use crate::common::models::MyError;

use crate::common::components::*;
use super::components::*;
use super::functions::*;
use crate::accounts::functions::load_data as load_summary_data;

#[component]
pub fn AccountDialogView() -> impl IntoView {
    let location = use_location();
    let (base, _set_base) = create_signal(get_base_page_path(location));
    let memo_params_map = use_params_map();
    
    let account_resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            if let Some(id) = value.get("id").cloned() {
                let id_clone = id.clone();
                load_summary_data(&id_clone).await
            } else {
                Err(MyError::ParseError(String::from(
                    "Could not parse id parameter from url",
                )))
            }
        },
    );

    view! {
        <Show
            when=move || { !memo_params_map.get().get("id").is_some() && !base.get().is_empty() }
            fallback=move || view! { <NullView/> }
        >
            <Suspense fallback=move || {
                view! {
                    <AccountDialog public_key="".to_string() path_base=base.get() account=None/>
                }
            }>

                {move || {
                    account_resource
                        .get()
                        .and_then(|res| res.ok())
                        .map(|data| {
                            view! {
                                <AccountDialog
                                    public_key="".to_string()
                                    path_base=base.get()
                                    account=Some(data.account)
                                />
                            }
                        })
                }}

            </Suspense>
        </Show>
    }
}
