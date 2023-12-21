use leptos::*;
use leptos_router::*;

use crate::transactions_page::load_data as load_transaction_data;

use super::functions::*;
use super::components::*;

#[component]
pub fn AccountDialogView() -> impl IntoView {
    let location = use_location();
    let base = get_base_page_path(location);
    let memo_params_map = use_params_map();
    let id = memo_params_map.with(|params| params.get("id").cloned()).unwrap_or_default();
    let id_for_other = id.clone();

    let account_resource = create_resource(|| (), move |_| {
        let id_clone_for_async = id.clone(); // Clone the ID for the async block
        async move { 
            load_data(&id_clone_for_async).await
        }
    });
    
    let trans_resource = create_resource(|| (), move |_| {
        let id_clone_for_async = id_for_other.clone(); // Clone the ID for the async block
        async move { 
            let limit = 3;
            load_transaction_data(limit, Some(id_clone_for_async)).await 
        }
    });

    view! {
        {move || match (account_resource.get(), trans_resource.get()) {
            (Some(Ok(a_res)), Some(Ok(t_res))) => view!{
                <AccountDialog path_base=base.to_owned() account=a_res.account transactions=t_res.data.transactions />
            },
            _ => view! { <span/>  }.into_view()
        }}
    }
}

