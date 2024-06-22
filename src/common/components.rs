use super::models::*;
use crate::{
    common::{constants::LINK_HOVER_STATE, functions::*},
    icons::*,
};
use leptos::{html::Div, *};
use leptos_meta::Script;
use leptos_router::*;
use web_sys::{window, Event, MouseEvent};

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
    #[prop(into)] id: String,
    #[prop(into)] query_str_key: String,
    labels: UrlParamSelectOptions,
) -> impl IntoView {
    let (query_val, set_query_val) = create_query_signal::<String>(query_str_key);
    let comparison_labels = labels.cases.clone();
    view! {
        <select
            class="ml-5 sm:ml-2 w-full sm:w-fit border-2 border-slate-200 focus:border-granola-orange focus:outline-none focus-visible:border-granola-orange active:border-granola-orange rounded-xl text-xs font-medium text-black p-2"
            id=id
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
pub fn PreSectionContainer(children: Children) -> impl IntoView {
    view! { <div class="flex flex-col md:flex-row mx-4 mb-4">{children()}</div> }
}

#[component]
pub fn NavLink<F>(nav_entry: NavEntry, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let location = use_location();
    let href = nav_entry.href.clone();
    let base_link_class = "nav-link whitespace-nowrap md:mx-1.5 my-6 mx-4 flex font-bold text-sm uppercase sm:tracking-normal md:tracking-tighter lg:tracking-normal sm:text-sm md:text-xs lg:text-sm items-center";
    let n_entry = nav_entry.clone();
    let get_link_class = create_memo(move |_| {
        let pathname = location.pathname.get();
        let tmp_class = if pathname.contains(&href) {
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
    view! {
        <a on:click=on_click class=move || link_class.get() href=nav_entry.href>
            {match nav_entry.icon {
                NavIcon::Blocks => view! { <BlockIcon/> },
                NavIcon::Transactions => view! { <TransactionIcon/> },
                NavIcon::Send => view! { <SendIcon/> },
                NavIcon::SNARKs => view! { <CheckCircleIcon/> },
                NavIcon::Staking => view! { <StakingIcon/> },
                NavIcon::Accounts => view! { <AccountIcon/> },
                NavIcon::ZKApps => view! { <ZKAppSymbol/> },
                NavIcon::Tokens => view! { <TokenSymbol/> },
                NavIcon::Addresses => view! { <AddressIcon/> },
                NavIcon::FeeTransfers => view! { <FeeTransferIcon/> },
                NavIcon::Analytics => view! { <AnalyticsIcon/> },
                NavIcon::More => view! { <MoreIcon/> },
            }}

            <span class="ml-0.5">{nav_entry.text}</span>
        </a>
    }
}

#[component]
pub fn TabLink(nav_entry: NavEntry) -> impl IntoView {
    let location = use_location();
    let pathname = move || location.pathname.get();
    let href = nav_entry.href.clone();
    let base_link_class = "tab mx-1 p-2 flex font-bold text-sm uppercase border-b border-b-2 whitespace-nowrap box-border";
    let disabled_link_class =
        "text-white border-transparent opacity-50 cursor-not-allowed pointer-events-none";
    let active_state = "active text-granola-orange border-granola-orange";
    let inactive_state = "inactive text-white border-transparent hover:border-white";
    view! {
        <a
            class=move || {
                format!(
                    "{} {} {}",
                    base_link_class,
                    if pathname().ends_with(&href) { active_state } else { inactive_state },
                    if nav_entry.disabled { disabled_link_class } else { "" },
                )
            }

            href=nav_entry.href
        >
            {match nav_entry.icon {
                NavIcon::Blocks => view! { <BlockIcon/> },
                NavIcon::Transactions => view! { <TransactionIcon/> },
                NavIcon::Send => view! { <SendIcon/> },
                NavIcon::SNARKs => view! { <CheckCircleIcon/> },
                NavIcon::Staking => view! { <StakingIcon/> },
                NavIcon::Accounts => view! { <AccountIcon/> },
                NavIcon::ZKApps => view! { <ZKAppSymbol/> },
                NavIcon::Tokens => view! { <TokenSymbol/> },
                NavIcon::Addresses => view! { <AddressIcon/> },
                NavIcon::FeeTransfers => view! { <FeeTransferIcon/> },
                NavIcon::Analytics => view! { <AnalyticsIcon/> },
                NavIcon::More => view! { <MoreIcon/> },
            }}

            <div class="ml-0.5">{nav_entry.text}</div>
            {match nav_entry.number_bubble {
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
pub fn TabbedPage(tabs: Vec<NavEntry>) -> impl IntoView {
    view! {
        <PreSectionContainer>
            <menu id="tabs" class="flex w-full overflow-x-auto">
                {tabs
                    .into_iter()
                    .map(|t| {
                        view! {
                            <li>
                                <TabLink nav_entry=t/>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </menu>
        </PreSectionContainer>
        <Outlet/>
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
                    let clipboard = window
                        .navigator()
                        .clipboard()
                        .expect("Could not get clipboard object");
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
                    true => view! { <CopiedIcon width=18/> },
                    false => view! { <ClipboardIcon width=18/> },
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
            src="https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js"
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
    variant: ColorVariant,
) -> impl IntoView {
    let mut container_class_str =
        "w-full p-4 rounded-lg flex flex-col justify-around items-stretch ".to_string();
    let base_class_str = "flex justify-center items-center";
    let mut value_class_str = " text-sm lg:text-base ".to_string();
    value_class_str.push_str(base_class_str);
    let mut label_class_str = " text-xs mt-2 ".to_string();
    label_class_str.push_str(base_class_str);
    match variant {
        ColorVariant::Blue => {
            container_class_str.push_str(" bg-blue/25 ");
            value_class_str.push_str(" text-blue ");
        }
        ColorVariant::Green => {
            container_class_str.push_str(" bg-green/25 ");
            value_class_str.push_str(" text-green ");
        }
        ColorVariant::Grey => {
            container_class_str.push_str(" bg-slate-400/25 ");
            value_class_str.push_str(" text-slate-400 ");
        }
        ColorVariant::Transparent => {
            container_class_str.push_str(" bg-transparent ");
            value_class_str.push_str(" text-inherit ");
        }
        ColorVariant::DarkBlue => {
            container_class_str.push_str(" bg-dark-blue/25 ");
            value_class_str.push_str(" text-dark-blue ");
        }
        ColorVariant::Orange => {
            container_class_str.push_str(" bg-amber-600/25 ");
            value_class_str.push_str(" text-inherit ");
        }
    }
    view! {
        <div class=container_class_str>
            <div class=value_class_str>{value}</div>
            <div class=label_class_str>{label}</div>
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
