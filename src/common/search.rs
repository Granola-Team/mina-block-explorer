use leptos::*;
use leptos_router::*;
use crate::icons::*;

#[component]
pub fn SearchBar() -> impl IntoView {

    let query_params_map = use_query_map();
    let navigate = use_navigate();
    let location = use_location();

    let params_map = query_params_map.get();
    let initial_query = params_map.get("query").cloned();

    let handle_input = move |event| {
        let input_value = event_target_value(&event);
        
        let pathname = location.pathname.get();
        let mut pm = query_params_map.get();
        pm.insert("query".to_string(), input_value);

        logging::log!("{}", pm.to_query_string());
        logging::log!("{}", pathname);
        
        navigate(&format!("{}{}", pathname, pm.to_query_string()), NavigateOptions {
            resolve: true,
            replace: false,
            scroll: false,
            state: State(None)
        });
    };

    view! {
        <div class="flex self-stretch relative flex items-stretch mx-[10%] mb-4">
            <input id="searchbar" 
                type="text" 
                placeholder=initial_query.unwrap_or("Search for public key".to_string())
                on:input=handle_input
                class="h-14 flex justify-start items-center text-base text-white pl-14 placeholder:text-slate-400 placeholder:font-medium placeholder:text-base focus:outline-none box-border w-full rounded-2xl bg-[#383B42]" />
                <span class="text-white absolute top-0 left-0 translate-x-3/4 translate-y-3/4"><SearchIcon width=22 /></span>
        </div>
    }
}