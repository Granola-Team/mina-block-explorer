use leptos::*;

pub struct SpotlightEntry {
    pub label: String,
    pub value: HtmlElement<html::AnyElement>,
    pub is_pill: bool,
}

#[component]
pub fn Spotlight(
    summary_items: Vec<SpotlightEntry>,
    username: String,
    public_key: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="@3xl:grid @3xl:grid-cols-[10rem_5rem_auto_10rem] @3xl:grid-rows-[2.5rem_2.5rem] @3xl:gap-x-[2rem] @3xl:h-auto flex flex-col items-center mt-16 bg-light-granola-orange rounded-3xl h-36">
            <div class="@3xl:col-start-2 @3xl:col-end-3 @3xl:row-start-1 @3xl:row-end-2 w-20 h-20 rounded-full bg-main-background flex justify-center items-center translate-y-[-25%] text-granola-orange">
                {children()}
            </div>
            <div class="@3xl:col-start-3 text-granola-orange text-base text-bold text-ellipsis w-10/12 overflow-hidden text-center @3xl:text-left">
                {public_key}
            </div>
            <div class="@3xl:col-start-3 @3xl:row-start-2 text-slate-400 text-sm">
                "Username: "{username}
            </div>
        </div>
        <div class="@3xl:grid @3xl:grid-cols-[10rem_auto_10rem] bg-white rounded-xl flex flex-col items-stretch mt-8 p-4">
            {summary_items.into_iter()
                .map(|entry| view! {
                    <SpotlightRow entry=entry />
                })
                .collect::<Vec<_>>()}

        </div>
    }
}

#[component]
fn SpotlightRow(entry: SpotlightEntry) -> impl IntoView {
    let value_class_str_base = "py-1 my-1 text-sm";

    let value_class_str = match entry.is_pill {
        true => format!(
            "{} {}",
            value_class_str_base.to_owned(),
            "p-1 rounded-full bg-light-granola-orange"
        ),
        false => format!(
            "{} {}",
            value_class_str_base.to_owned(),
            "w-3/4 text-ellipsis overflow-hidden"
        ),
    };

    view! {
        <div class="@3xl:col-start-2 @3xl:col-end-3 flex flex-col items-start md:flex-row md:items-baseline md:justify-start">
            <span class="w-1/4 text-slate-400 text-sm whitespace-nowrap">{entry.label}:</span>
            <span class=value_class_str>{entry.value}</span>
        </div>
    }
}
