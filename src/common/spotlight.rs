use leptos::*;

use crate::common::models::*;

pub struct SpotlightEntry {
    pub label: String,
    pub value: String,
    pub pill_variant: Option<PillVariant>,
}

#[component]
pub fn Spotlight(
    summary_items: Vec<SpotlightEntry>,
    meta: String,
    id: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div id="spotlight-heading" class="@3xl:grid @3xl:grid-cols-[10rem_5rem_auto_10rem] @3xl:grid-rows-[2.5rem_2.5rem] @3xl:gap-x-[2rem] @3xl:h-auto flex flex-col items-center mt-16 bg-light-granola-orange rounded-3xl h-36">
            <div id="spotlight-icon" class="@3xl:col-start-2 @3xl:col-end-3 @3xl:row-start-1 @3xl:row-end-2 w-20 h-20 rounded-full bg-main-background flex justify-center items-center translate-y-[-25%] text-granola-orange">
                {children()}
            </div>
            <div id="spotlight-id" class="@3xl:col-start-3 text-granola-orange text-base text-bold text-ellipsis w-10/12 overflow-hidden text-center @3xl:text-left">
                {id}
            </div>
            <div id="spotlight-meta" class="@3xl:col-start-3 @3xl:row-start-2 text-slate-400 text-sm">
                {meta}
            </div>
        </div>
        <div id="spotlight-info" class="@3xl:grid @3xl:grid-cols-[10rem_auto_10rem] bg-white rounded-xl flex flex-col items-stretch mt-8 p-4">
            {summary_items.into_iter()
                .map(|entry| view! { <SpotlightRow entry=entry /> })
                .collect::<Vec<_>>()
                .as_slice()
                .chunks(2)
                .map(|chunk| {
                    view! {
                        <div class="@3xl:col-start-2 @3xl:col-end-3 @7xl:flex">
                            { chunk.first() } {chunk.get(1)}
                        </div>
                    }
                }) // Wrap each chunk in a view
                .collect::<Vec<_>>()}

        </div>
    }
}

#[component]
fn SpotlightRow(entry: SpotlightEntry) -> impl IntoView {
    let value_class_str_base = "p-1 my-1 text-sm";
    let pill_class_str_base = format!(
        "{} {}",
        value_class_str_base, "px-4 rounded-full text-white"
    );

    let value_class_str = match entry.pill_variant {
        Some(PillVariant::Green) => {
            format!("{} {}", pill_class_str_base.to_owned(), "bg-pill-green")
        }
        Some(PillVariant::Blue) => format!("{} {}", pill_class_str_base.to_owned(), "bg-pill-blue"),
        Some(PillVariant::Orange) => {
            format!("{} {}", pill_class_str_base.to_owned(), "bg-granola-orange")
        }
        None => format!(
            "{} {}",
            value_class_str_base.to_owned(),
            "w-1/2 text-ellipsis overflow-hidden"
        ),
    };

    view! {
        <div class="flex flex-row items-start items-baseline justify-start w-full @7xl:max-w-[50%]">
            <span class="w-1/4 min-w-[150px] text-slate-400 text-sm whitespace-nowrap">{entry.label}:</span>
            <span class=value_class_str>{entry.value}</span>
        </div>
    }
}
