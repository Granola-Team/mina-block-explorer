use crate::{common::components::*, icons::*};
use leptos::*;
use leptos_router::*;
use leptos_use::signal_debounced;

#[component]
pub fn SearchBar(
    #[prop(default="Exact search for public key".to_string())] placeholder: String,
) -> impl IntoView {
    view! {
        <div class="flex self-stretch relative flex items-stretch md:mx-[10%] mb-4">
            <SearchInput placeholder=placeholder/>
        </div>
    }
}

#[component]
fn SearchInput(placeholder: String) -> impl IntoView {
    let (query, set_query) = create_query_signal::<String>("query");

    let (input, set_input) = create_signal(None);
    let debounced: Signal<Option<String>> = signal_debounced(input, 500.0);

    create_effect(move |last_query| match (query.get(), last_query) {
        (Some(query_str), Some(Some(last_query_str))) => {
            if query_str != last_query_str {
                logging::log!("Setting input from query {}", query_str);
                set_input.set(Some(query_str.clone()));
                return Some(query_str);
            }
            Some(last_query_str)
        }
        (Some(query_str), Some(None)) => {
            logging::log!("Setting input from query {}", query_str);
            set_input.set(Some(query_str.clone()));
            Some(query_str)
        }
        _ => None,
    });

    create_effect(move |prev_value| match (debounced.get(), prev_value) {
        (Some(input_value), Some(Some(prev_value_str))) => {
            if input_value != prev_value_str {
                logging::log!("Setting query from input {}", input_value);
                match !input_value.is_empty() {
                    true => set_query.set(Some(input_value.to_string())),
                    false => set_query.set(None),
                }
                return Some(input_value);
            }
            Some(input_value)
        }
        (Some(input_value), Some(None)) => {
            logging::log!("Setting query from input {}", input_value);
            match !input_value.is_empty() {
                true => set_query.set(Some(input_value.to_string())),
                false => set_query.set(None),
            }
            Some(input_value)
        }
        _ => None,
    });

    view! {
        <input
            id="searchbar"
            type="text"
            on:input=move |event| {
                let text = event_target_value(&event);
                match !text.is_empty() {
                    true => set_query.set(Some(text)),
                    false => set_query.set(None),
                }
            }

            prop:value=move || query.get().unwrap_or("".to_string())
            placeholder=move || query.get().unwrap_or(placeholder.clone())
            on:input=move |event| set_input.update(|e| *e = Some(event_target_value(&event)))
            class="h-14 flex justify-start items-center text-base text-white pl-14 placeholder:text-slate-400 placeholder:font-medium placeholder:text-base focus:outline-none box-border w-full rounded-2xl bg-[#383B42]"
        />
        <span class="text-white absolute top-0 left-0 translate-x-3/4 translate-y-3/4">
            <SearchIcon width=22/>
        </span>
    }
}

#[component]
pub fn TitledSearchBar(
    title: String,
    subtext: String,
    search_placeholder: String,
) -> impl IntoView {
    view! {
        <PreSectionContainer>
            <div class="pl-8 md:pl-0 flex flex-col md:pr-4">
                <span class="text-white font-bold text-2xl p-1 whitespace-nowrap">{title}</span>
                <span class="text-white font-medium text-base p-1 whitespace-nowrap">
                    {subtext}
                </span>
            </div>
            <div class="mx-2 my-2 md:mx-0 md:w-full relative align-stretch flex items-center">
                <SearchInput placeholder=search_placeholder/>
            </div>
        </PreSectionContainer>
    }
}
