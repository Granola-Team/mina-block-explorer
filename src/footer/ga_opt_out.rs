use crate::icons::*;
use leptos::*;
use leptos_use::storage::{use_local_storage};
use leptos_use::utils::FromToStringCodec;

#[component]
pub fn GAOptOut() -> impl IntoView {
    let (flag, set_flag, _remove_flag) = use_local_storage::<bool, FromToStringCodec>("ga-opt-out");
    let (first_load, set_first_load) = create_signal(true);

    create_effect(move |_| {
        match (flag.get(), first_load.get()) {
            (_, true) => {
                set_first_load.set(false);
            }
            (_, false) => {
                // so that analytics can be conditionally loaded back into index.html
                let _ = window().location().reload();
            }
        }
    });

    view! {
        <button
            class="ml-1 sm:ml-4 flex items-center text-white text-xs uppercase hover:text-granola-orange hover:underline"
            on:click=move |_| set_flag.update(|f| *f = !*f)
            title=move || match flag.get() {
                true => "You have opted out of Google Analytics tracking",
                false => "Google Analytics tracking is active",
            }
        >

            {move || match flag.get() {
                true => view! { <NoSymbol width=12/> },
                false => view! { <CheckCircleIcon width=12/> },
            }}

            <div class="ml-1 whitespace-nowrap overflow-hidden text-ellipsis">
                "Data & Tracking"
            </div>
        </button>
    }
}
