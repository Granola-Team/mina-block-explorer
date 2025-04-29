use super::models::*;
use crate::{
    common::{constants::*, functions::*},
    icons::*,
};
use heck::ToKebabCase;
use leptos::{html::Div, *};
use leptos_meta::Script;
use leptos_router::{create_query_signal, *};
use leptos_use::{use_debounce_fn_with_options, DebounceOptions};
use std::collections::HashMap;
use web_sys::{window, Event, MouseEvent};

#[component]
pub fn NextBlockPage<T: HasBlockHeight>(
    data: Vec<Option<T>>,
    row_limit: Option<u64>,
) -> impl IntoView {
    let (_, set_height) = create_query_signal_with_options::<i64>(
        QUERY_PARAM_HEIGHT,
        NavigateOptions {
            scroll: false,
            ..Default::default()
        },
    );
    let (row_limit_sig, set_row_limit) = create_query_signal_with_options::<u64>(
        QUERY_PARAM_ROW_LIMIT,
        NavigateOptions {
            scroll: false,
            ..Default::default()
        },
    );
    let mut last_block_height = None;
    let mut first_block_height = None;
    if let Some(Some(first_row)) = data.first() {
        first_block_height = first_row.block_height();
    }
    if let Some(Some(last_row)) = data.last() {
        last_block_height = last_row.block_height();
    }
    let not_pageable = first_block_height
        .zip(last_block_height)
        .map(|(first_height, last_height)| last_height == first_height)
        .unwrap_or_default();
    view! {
        <div class="w-full flex justify-center items-center p-4">
            <Button
                style_variant=ButtonStyleVariant::Tertiary
                text="Load Next"
                on_click=move |_| {
                    if not_pageable {
                        set_row_limit.set(row_limit_sig.get().map(|rl| rl * 2).or(Some(50)))
                    } else {
                        set_height.set(last_block_height)
                    }
                }
                class_str="ml-2"
                disabled=data.len() as u64 != row_limit.unwrap_or(TABLE_ROW_LIMIT)
            />
        </div>
    }
}

#[component]
pub fn RowLimit() -> impl IntoView {
    view! {
        <UrlParamSelectMenu
            label="Rows"
            id="row-limit"
            query_str_key="row-limit"
            labels=UrlParamSelectOptions {
                is_boolean_option: false,
                cases: vec![
                    "25".to_string(),
                    "50".to_string(),
                    "100".to_string(),
                    "250".to_string(),
                    "500".to_string(),
                    "1000".to_string(),
                ],
            }
        />
    }
}

#[component]
pub fn Button<F>(
    on_click: F,
    #[prop(into)] text: String,
    #[prop(optional, default=ButtonStyleVariant::Primary)] style_variant: ButtonStyleVariant,
    #[prop(optional, default = false)] disabled: bool,
    #[prop(optional, into, default=String::new())] class_str: String,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let button_base_styles = "text-sm rounded-md p-2 h-9 font-semibold flex justify-center items-center border border-granola-orange border-[1px]";
    let mut button_variant_styles = format!(
        "{} {}",
        button_base_styles,
        get_button_style_variation(&style_variant)
    );
    button_variant_styles = match disabled {
        true => format!(
            "{} {}",
            button_variant_styles,
            "bg-slate-100 text-slate-400 border-slate-100 hover:cursor-not-allowed"
        ),
        false => button_variant_styles,
    };
    view! {
        <button disabled=disabled on:click=on_click class=class_str + " " + &button_variant_styles>
            {text}
        </button>
    }
}

#[component]
pub fn ControlledInput<T>(
    #[prop(into, default = "controlled-input-id".to_string())] id: String,
    #[prop(into, default = "controlled-input-name".to_string())] name: String,
    #[prop(into)] input_type: String,
    #[prop(into)] disabled_sig: Signal<bool>,
    #[prop(into)] value_sig: Memo<Option<T>>,
    #[prop(into)] setter_sig: SignalSetter<Option<String>>,
    #[prop(optional)] input_class: Option<String>,
    #[prop(optional)] number_props: Option<HashMap<String, String>>,
) -> impl IntoView
where
    T: From<u32> + ToString + Clone + 'static,
{
    let unwrapped_input_class = input_class.unwrap_or(DEFAULT_INPUT_STYLES.to_string());
    let input_element: NodeRef<html::Input> = create_node_ref();
    let (handle_input_sig, _) = create_signal(use_debounce_fn_with_options(
        move || {
            let v = input_element
                .get()
                .expect("<input/> should be mounted")
                .value();
            setter_sig.set(Some(v));
        },
        DEFAULT_USER_INPUT_DEBOUNCE_INTERNVAL,
        DebounceOptions::default(),
    ));

    view! {
        <input
            data-test=id.to_string() + "-input"
            id=id
            type=input_type
            on:input=move |_| {
                let handle = handle_input_sig.get_untracked();
                handle();
            }

            disabled=move || disabled_sig.get()
            name=name
            value=value_sig.get().map(|v| v.to_string()).unwrap_or_default()
            step=number_props
                .clone()
                .and_then(|props| props.get("step").cloned())
                .unwrap_or_default()
            max=number_props.clone().and_then(|props| props.get("max").cloned()).unwrap_or_default()
            min=number_props.and_then(|props| props.get("min").cloned()).unwrap_or_default()
            class=unwrapped_input_class
            node_ref=input_element
        />
    }
}

#[component]
pub fn SummaryItem(
    #[prop(into)] label: String,
    #[prop(into)] value: Option<String>,
    #[prop(into)] id: String,
    #[prop(optional, into)] _imgsrc: String,
) -> impl IntoView {
    view! {
        <div class="h-24 w-full p-4 grid gap-2 grid-cols-1 bg-white rounded-md">
            <div class="font-bold text-xl flex justify-start items-end" id=id.clone()>

                {{
                    match value {
                        Some(str_val) => view! { <span>{str_val}</span> }.into_view(),
                        None => data_placeholder().into_view(),
                    }
                }}

            </div>
            <label
                class="row-start-2 text-sm text-slate-500 font-semibold flex justify-start items-start"
                for=id.clone()
            >
                {label}
            </label>
        </div>
    }
}

#[component]
pub fn SubSectionContainer(children: Children) -> impl IntoView {
    view! { <div class="grid grid-cols-1 md:grid-cols-2 gap-4">{children()}</div> }
}

#[component]
pub fn AppSection(children: Children) -> impl IntoView {
    view! { <section class="md:rounded-lg bg-table-section mb-4">{children()}</section> }
}

#[component]
pub fn AppHeading(#[prop(into)] heading: String) -> impl IntoView {
    view! {
        <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">
            {heading}
        </h1>
    }
}

#[component]
pub fn Checkbox<F>(#[prop(into)] label: String, value: bool, handle_change: F) -> impl IntoView
where
    F: Fn(Event) + 'static,
{
    view! {
        <label class="text-sm grid grid-cols-[1em_auto] gap-1 font-semibold checked:text-granola-orange">
            <input
                on:change=handle_change
                prop:checked=value
                name="checkbox"
                type="checkbox"
                class="accent-granola-orange"
            />
            {label}
        </label>
    }
}

#[component]
pub fn UrlParamSelectMenu(
    #[prop(into, optional)] label: Option<String>,
    #[prop(into)] id: String,
    #[prop(into)] query_str_key: String,
    labels: UrlParamSelectOptions,
) -> impl IntoView {
    let (query_val, set_query_val) = create_query_signal::<String>(query_str_key);
    let comparison_labels = labels.cases.clone();
    view! {
        {label
            .map(|l| {
                view! { <label class="text-xs font-medium text-black">{l}</label> }.into_view()
            })
            .unwrap_or_else(|| ().into_view())}
        <select
            class="ml-5 sm:ml-2 w-full sm:w-fit border-2 border-slate-200 focus:border-granola-orange focus:outline-none focus-visible:border-granola-orange active:border-granola-orange rounded-xl text-xs font-medium text-black p-2"
            id=id.to_string()
            data-test=id
            on:change=move |ev| {
                if labels.is_boolean_option {
                    match event_target_value(&ev) {
                        val if val == comparison_labels[0] => {
                            set_query_val.set(Some("true".to_string()))
                        }
                        val if val == comparison_labels[1] => {
                            set_query_val.set(Some("false".to_string()))
                        }
                        val => logging::log!("unknown value {}", val),
                    }
                } else {
                    set_query_val.set(Some(event_target_value(&ev)))
                }
            }
        >

            {if labels.is_boolean_option {
                let is_true_case = move || match query_val.get() {
                    Some(ref val) if val == "true" => true,
                    Some(ref val) if val == "false" => false,
                    Some(_) | None => true,
                };
                view! {
                    <option class="text-xs font-mono" selected=is_true_case()>
                        {labels.cases[0].clone()}
                    </option>
                    <option class="text-xs font-mono" selected=!is_true_case()>
                        {labels.cases[1].clone()}
                    </option>
                }
                    .into_view()
            } else {
                let is_selected = move |selection| {
                    query_val.get().filter(|q| q == &selection).is_some()
                };
                labels
                    .cases
                    .into_iter()
                    .map(|c| {
                        view! {
                            <option class="text-xs font-mono" selected=is_selected(c.clone())>
                                {c}
                            </option>
                        }
                            .into_view()
                    })
                    .collect::<Vec<_>>()
                    .into_view()
            }}

        </select>
    }
}

#[component]
pub fn ErrorView<E: std::fmt::Debug>(err: E) -> impl IntoView {
    view! { <div class="error">{format!("Error: {:#?}", err)}</div> }
}

#[component]
pub fn PageContainer(children: Children) -> impl IntoView {
    let el = create_node_ref::<Div>();

    view! {
        <div
            node_ref=el
            class="grid grid-cols-1 auto-rows-min bg-secondary-background p-2 sm:px-0 grow min-h-[85vh]"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn NavEntryIcon(nav_entry: NavEntry) -> impl IntoView {
    view! {
        {match nav_entry.icon {
            NavIcon::Blocks => view! { <BlockIcon /> },
            NavIcon::Transactions => view! { <TransactionIcon /> },
            NavIcon::Send => view! { <SendIcon /> },
            NavIcon::SNARKs => view! { <CheckCircleIcon /> },
            NavIcon::Staking => view! { <StakingIcon /> },
            NavIcon::Accounts => view! { <AccountIcon /> },
            NavIcon::ZKApps => view! { <ZKAppSymbol /> },
            NavIcon::Tokens => view! { <TokenSymbol /> },
            NavIcon::Addresses => view! { <AddressIcon /> },
            NavIcon::FeeTransfers => view! { <FeeTransferIcon /> },
            NavIcon::Analytics => view! { <AnalyticsIcon /> },
            NavIcon::More => view! { <MoreIcon /> },
            NavIcon::Delegates => view! { <DelegateIcon /> },
            NavIcon::Leaderboard => view! { <LeaderboardIcon /> },
        }}
    }
}

#[component]
pub fn PreSectionContainer(children: Children) -> impl IntoView {
    view! { <div class="flex flex-col md:flex-row mx-4 mb-4">{children()}</div> }
}

#[component]
pub fn NavLink<F>(nav_entry: NavEntry, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let location = use_location();
    let base_link_class = "nav-link whitespace-nowrap md:mx-1.5 my-6 mx-4 flex font-bold text-sm uppercase sm:tracking-normal md:tracking-tighter lg:tracking-normal sm:text-sm md:text-xs lg:text-sm items-center ";
    let n_entry = nav_entry.clone();
    let get_link_class = create_memo(move |_| {
        let pathname = location.pathname.get();
        let tmp_class = if n_entry.is_match(&pathname) {
            format!(
                "{} {} {}",
                base_link_class, LINK_HOVER_STATE, "text-granola-orange"
            )
        } else {
            format!("{} {} {}", base_link_class, LINK_HOVER_STATE, "text-white")
        };
        if n_entry.disabled {
            format!(
                "{} {}",
                tmp_class, "opacity-50 cursor-not-allowed pointer-events-none"
            )
        } else {
            tmp_class
        }
    });
    let (link_class, set_link_class) = create_signal(get_link_class.get_untracked());
    create_effect(move |_| {
        set_link_class.set(get_link_class.get());
    });
    let text = nav_entry.text.clone();
    let href = nav_entry.href.clone();
    view! {
        <a on:click=on_click class=move || link_class.get() href=href>
            <NavEntryIcon nav_entry />
            <span class="ml-0.5">{text}</span>
        </a>
    }
}

#[component]
pub fn TabLink(nav_entry: NavEntry) -> impl IntoView {
    let location = use_location();
    let pathname = move || location.pathname.get();
    let base_link_class = "tab mx-1 p-2 flex font-bold text-sm uppercase border-b border-b-2 whitespace-nowrap box-border";
    let disabled_link_class =
        "text-white border-transparent opacity-50 cursor-not-allowed pointer-events-none";
    let active_state = "active text-granola-orange border-granola-orange";
    let inactive_state = "inactive text-white border-transparent hover:border-white";
    let n_entry = nav_entry.clone();
    let text = nav_entry.text.clone();
    let number_bubble = nav_entry.number_bubble;
    let href = nav_entry.href.clone();
    view! {
        <a
            data-test=format!("{}-tab", nav_entry.text.as_str().to_lowercase().to_kebab_case())
            class=move || {
                format!(
                    "{} {} {}",
                    base_link_class,
                    if n_entry.is_match(&pathname()) { active_state } else { inactive_state },
                    if n_entry.disabled { disabled_link_class } else { "" },
                )
            }
            href=href
        >
            <NavEntryIcon nav_entry />
            <div class="ml-0.5">{text}</div>
            {match number_bubble {
                Some(number) => {
                    view! {
                        <div class="number-bubble ml-1 px-1 flex text-granola-orange text-xs bg-granola-orange/25 justify-center items-center rounded-full">
                            {number}
                        </div>
                    }
                        .into_view()
                }
                None => ().into_view(),
            }}

        </a>
    }
}

#[component]
pub fn TabbedPage(
    tabs: Vec<NavEntry>,
    #[prop(default = false)] exclude_outlet: bool,
) -> impl IntoView {
    view! {
        <PreSectionContainer>
            <menu id="tabs" class="flex w-full overflow-x-auto">
                {tabs
                    .into_iter()
                    .map(|t| {
                        view! {
                            <li>
                                <TabLink nav_entry=t />
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </menu>
        </PreSectionContainer>
        {if !exclude_outlet {
            view! { <Outlet /> }
        } else {
            ().into_view()
        }}
    }
}

#[component]
pub fn CopyToClipboard(children: Children) -> impl IntoView {
    let element: NodeRef<leptos::html::Div> = create_node_ref();
    let (copied, set_copied) = create_signal(false);
    let (text_color, set_text_color) = create_signal("text-slate-700");
    create_effect(move |_| {
        if copied.get() {
            set_text_color.set("text-green")
        } else {
            set_text_color.set("text-slate-700")
        }
    });

    view! {
        <div class="relative group w-fit max-w-full text-ellipsis overflow-hidden" node_ref=element>
            <span
                on:click=move |_| {
                    let mut value = element.get().expect("<div> element").inner_text();
                    value.retain(|c| !c.is_whitespace());
                    let window = window().expect("no global `window` exists");
                    let clipboard = window.navigator().clipboard();
                    let _ = clipboard.write_text(&value);
                    set_copied.set(true);
                    logging::log!("copied value '{}'", value);
                }

                on:mouseleave=move |_| {
                    logging::log!("mouse exited copytoclipboard");
                    set_copied.set(false);
                }

                class=move || {
                    format!(
                        "hidden group-hover:block rounded-sm absolute top-0 right-0 bottom-0 p-0.5 bg-white z-10 cursor-pointer {}",
                        text_color.get(),
                    )
                }
            >

                {move || match copied.get() {
                    true => view! { <CopiedIcon width=18 /> },
                    false => view! { <ClipboardIcon width=18 /> },
                }}

            </span>
            {children()}
        </div>
    }
}

#[component]
fn AnalyticsContainer(children: Children, #[prop(into)] span: String) -> impl IntoView {
    let class_str = format!(
        "bg-secondary-background rounded-lg flex justify-center items-center {}",
        span
    );
    view! { <div class=class_str>{children()}</div> }
}

#[component]
pub fn AnalyticsSmContainer(children: Children) -> impl IntoView {
    view! {
        <AnalyticsContainer span="analytics-sm col-span-1 md:col-span-2">
            {children()}
        </AnalyticsContainer>
    }
}

#[component]
pub fn AnalyticsLgContainer(children: Children) -> impl IntoView {
    view! {
        <AnalyticsContainer span="analytics-lg col-span-1 md:col-span-2">
            {children()}
        </AnalyticsContainer>
    }
}

#[component]
pub fn AnalyticsXLContainer(children: Children) -> impl IntoView {
    view! {
        <AnalyticsContainer span="analytics-lg col-span-1 md:col-span-4">
            {children()}
        </AnalyticsContainer>
    }
}

#[component]
pub fn AnalyticsLayout(children: Children) -> impl IntoView {
    view! {
        <Script
            src="https://cdn.jsdelivr.net/npm/echarts@5.5.0/dist/echarts.min.js"
            crossorigin="anonymous"
        />
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-2 md:gap-4 p-2 md:p-8">
            {children()}
        </div>
    }
}

#[component]
pub fn AnalyticsSimpleInfo(
    value: HtmlElement<html::AnyElement>,
    label: HtmlElement<html::AnyElement>,
    #[prop(optional, into)] subtext: Option<String>,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div class="w-full p-4 rounded-lg flex flex-col md:flex-row justify-center items-stretch">
            <div class="w-full md:w-1/2 flex flex-wrap flex-col justify-center mx-4">
                <div class="min-w-full flex justify-center md:justify-end label text-lg font-semibold text-ellipsis overflow-hidden">
                    {label}
                </div>
                {subtext
                    .map(|t| {
                        view! {
                            <div class="min-w-full flex justify-center md:justify-end subtext text-sm font-medium text-slate-400">
                                {t}
                            </div>
                        }
                    })}
            </div>
            <div
                data-test="analytics-simple-info"
                class="w-full md:w-1/2 flex grow-0 justify-center md:justify-start items-center mx-4 text-6xl font-bold"
                id=id.unwrap_or("simple-info".to_string())
            >
                {value}
            </div>
        </div>
    }
}

#[component]
pub fn CodeBlock(children: Children) -> impl IntoView {
    view! {
        <div class="w-full overflow-x-auto">
            <pre class="p-4 border-box stretch w-fit border border-[#DADCE0] rounded-md">
                {children()}
            </pre>
        </div>
    }
}

#[component]
pub fn NotFound(message: Option<String>) -> impl IntoView {
    view! {
        <div class="bg-slate-100 flex justify-center items-start px-16 py-32">
            <div class="text-slate-400 text-2xl font-extrabold">
                {message.unwrap_or("Page Not Found :(".to_string())}
            </div>
        </div>
    }
}

#[component]
pub fn ZkAppDetailTr(children: Children) -> impl IntoView {
    view! { <tr class="w-full flex flex-col lg:flex-row justify-start">{children()}</tr> }
}

#[component]
pub fn ZkAppDetailTh(children: Children) -> impl IntoView {
    view! {
        <th class="flex justify-start items-start my-1 py-1 text-xs md:text-sm whitespace-nowrap w-36 md:w-40 min-w-36 md:min-w-40 font-normal text-slate-400">
            {children()}
        </th>
    }
}

#[component]
pub fn ZkAppDetailTd(children: Children) -> impl IntoView {
    view! {
        <td class="flex justify-start items-center my-1 py-1 text-left text-xs md:text-sm whitespace-nowrap">
            {children()}
        </td>
    }
}
