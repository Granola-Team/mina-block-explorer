use leptos::*;
use leptos_router::*;

use crate::api_models::MyError;

use super::functions::*;
use super::components::*;

#[component]
pub fn AccountDialogView() -> impl IntoView {
    let location = use_location();
    let base = get_base_page_path(location);
    let memo_params_map = use_params_map();

    let account_resource = create_resource(move || memo_params_map.get(), |value| {
        async move {
            if let Some(id) = value.get("id").cloned() {
                
                let id_clone = id.clone();
                load_data(&id_clone).await
            } else {
                Err(MyError::ParseError(String::from("Could not parse id parameter from url")))
            }
        }
    });
    
    

    view! {
        {move || match account_resource.get() {
            Some(Ok(res)) => view!{
                <AccountDialog path_base=base.to_owned() account=res.account />
            },
            _ => view! { <span/>  }.into_view()
        }}
    }
}

