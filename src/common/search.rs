use crate::icons::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn SearchBar(#[prop(default="Exact search for public key".to_string())] placeholder: String) -> impl IntoView {
    view! {
        <div class="flex self-stretch relative flex items-stretch md:mx-[10%] mb-4">
            <SearchInput placeholder=placeholder/>
        </div>
    }
}

#[component]
fn SearchInput(placeholder: String) -> impl IntoView {
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

        navigate(
            &format!("{}{}", pathname, pm.to_query_string()),
            NavigateOptions {
                resolve: true,
                replace: false,
                scroll: false,
                state: State(None),
            },
        );
    };
    view! {
        <input id="searchbar"
            type="text"
            placeholder=initial_query.unwrap_or(placeholder)
            on:input=handle_input
            class="h-14 flex justify-start items-center text-base text-white pl-14 placeholder:text-slate-400 placeholder:font-medium placeholder:text-base focus:outline-none box-border w-full rounded-2xl bg-[#383B42]" />
            <span class="text-white absolute top-0 left-0 translate-x-3/4 translate-y-3/4"><SearchIcon width=22 /></span>
    }
}

#[component]
pub fn TitledSearchBar(title: String, subtext: String, search_placeholder:String) -> impl IntoView {
    view! {
        <div class="flex flex-col md:flex-row md:px-[10vw] mb-4">
            <div class="pl-8 md:pl-0 flex flex-col md:pr-4">
                <span class="text-white font-bold text-2xl p-1 whitespace-nowrap">{title}</span>
                <span class="text-white font-medium text-base p-1 whitespace-nowrap">{subtext}</span>
            </div>
            <div class="mx-2 my-2 md:mx-0 md:w-full relative align-stretch flex items-center">
                <SearchInput placeholder=search_placeholder/>
            </div>
        </div>
    }
}
