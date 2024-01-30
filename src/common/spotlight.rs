use leptos::*;

use crate::common::functions::*;
use crate::common::models::*;
use crate::common::components::*;

pub struct SpotlightEntry {
    pub label: String,
    pub value: Option<String>,
    pub pill_variant: Option<PillVariant>,
    pub copiable: bool
}

#[component]
pub fn SpotlightSection(
    header: String,
    spotlight_items: Vec<SpotlightEntry>,
    id: Option<String>,
    meta: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <section id="spotlight-section" class="@container md:col-start-2 md:col-end-3 md:rounded-lg bg-table-section p-0 md:p-4 mb-2">
            <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">{header}</h1>
            <Spotlight spotlight_items=spotlight_items id=id meta=meta>
                {children()}
            </Spotlight>
        </section>
    }
}

#[component]
fn Spotlight(
    spotlight_items: Vec<SpotlightEntry>,
    meta: Option<String>,
    id: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div id="spotlight-heading" class="@3xl:grid @3xl:grid-cols-[10rem_5rem_auto_10rem] @3xl:grid-rows-[2.5rem_2.5rem] @3xl:gap-x-[2rem] @3xl:h-auto flex flex-col items-center mt-16 bg-light-granola-orange rounded-3xl h-36">
            <div id="spotlight-icon" class="@3xl:col-start-2 @3xl:col-end-3 @3xl:row-start-1 @3xl:row-end-2 w-20 h-20 rounded-full bg-main-background flex justify-center items-center translate-y-[-25%] text-granola-orange">
                {children()}
            </div>
            <div id="spotlight-id" class="@3xl:col-start-3 text-granola-orange text-base text-bold text-ellipsis w-10/12 overflow-hidden text-center @3xl:text-left">
                {match id {
                    Some(i) => view! { <CopyToClipboard>{i}</CopyToClipboard> }.into_view(),
                    None => data_placeholder().into_view()
                }}
            </div>
            <div id="spotlight-meta" class="@3xl:col-start-3 @3xl:row-start-2 text-slate-400 text-sm max-w-80">
                {match meta {
                    Some(m) => view! { <span>{m}</span> }.into_view(),
                    None => data_placeholder().into_view()
                }}
            </div>
        </div>
        <table class="@3xl:mx-[10rem] bg-white rounded-xl mt-8 p-4 table-fixed flex flex-wrap">
            {spotlight_items.into_iter()
                .map(|entry| view! {
                    <tr class="h-9 w-full @7xl:w-1/2 overflow-hidden flex">
                        <SpotlightRow entry=entry />
                    </tr>
                })
                .collect::<Vec<_>>()}
        </table>
    }
}

#[component]
fn SpotlightRow(entry: SpotlightEntry) -> impl IntoView {
    let value_class_str_base = "text-sm text-ellipsis overflow-hidden";
    let pill_class_str_base = format!(
        "{} {}",
        value_class_str_base, "px-2 py-0.5 rounded-full text-white"
    );

    let value_class_str = match entry.pill_variant {
        Some(pill_variant) => format!(
            "{} {}",
            pill_class_str_base,
            pill_variant_to_style_str(pill_variant)
        ),
        None => value_class_str_base.to_string(),
    };
    let th_td_class_base = "flex justify-start items-center m-1 p-1 text-left";

    view! {
        <th class=format!("{} {}",th_td_class_base,"w-36 min-w-36 text-sm font-normal text-slate-400 whitespace-nowrap")>{entry.label}:</th>
        <td class=format!("{} {}",th_td_class_base,"block w-fit text-ellipsis overflow-hidden whitespace-nowrap")>
            {match entry.value {
                Some(val) => match entry.copiable {
                    true => view! {
                        <CopyToClipboard> 
                            <span class=value_class_str>{val}</span>
                        </CopyToClipboard>
                    }.into_view(),
                    false => view! { 
                        <span class=value_class_str>{val}</span>
                    }.into_view()
                },
                None => data_placeholder().into_view()
            }}

        </td>
    }
}
