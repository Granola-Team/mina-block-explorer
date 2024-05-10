use crate::{
    common::{components::*, constants::*},
    icons::*,
};
use leptos::*;

#[component]
pub fn GlobalSearchBar() -> impl IntoView {
    let input_element: NodeRef<html::Input> = create_node_ref();
    let (value, set_value) = create_signal("".to_string());

    let navigate = leptos_router::use_navigate();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        match value.get() {
            val if val.starts_with("B62q") => {
                navigate(&format!("/addresses/accounts/{}", val), Default::default());
                set_value.set("".to_string());
            }
            val if val.starts_with("3N") => {
                navigate(&format!("/blocks/{}", val), Default::default());
                set_value.set("".to_string());
            }
            val if val.starts_with("Ckp") => {
                navigate(&format!("/commands/{}", val), Default::default());
                set_value.set("".to_string());
            }
            val if val.chars().all(char::is_numeric) => {
                navigate(
                    &format!("/staking-ledgers?epoch={}", val),
                    Default::default(),
                );
                set_value.set("".to_string());
            }
            _ => {}
        }
    };

    view! {
        <PreSectionContainer>
            <div class="mx-2 my-2 md:mx-0 md:w-full -mt-2 relative align-stretch flex items-center">
                <form class="flex grow" on:submit=on_submit>
                    <input
                        id="searchbar"
                        type="text"
                        on:input=move |ev| {
                            set_value.set(event_target_value(&ev));
                        }

                        prop:value=value
                        placeholder=GLOBAL_SEARCH_PLACEHOLDER_TEXT
                        class="h-14 flex justify-start items-center text-base text-white pl-14 placeholder:text-slate-400 placeholder:font-medium placeholder:text-base focus:outline-none box-border w-full rounded-2xl bg-[#383B42]"
                        node_ref=input_element
                    />
                </form>
                <span class="text-white absolute top-0 left-0 translate-x-3/4 translate-y-3/4">
                    <SearchIcon width=22/>
                </span>
            </div>
        </PreSectionContainer>
    }
}
