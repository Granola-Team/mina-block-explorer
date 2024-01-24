use super::functions::*;
use super::models::*;
use crate::icons::*;
use leptos::{web_sys::*, *};
use leptos_router::*;

pub enum SubSectionPosition {
    Left,
    Right,
}

#[component]
pub fn SubSectionContainer(children: Children) -> impl IntoView {
    view! {
        <div class="md:col-start-2 md:col-end-3 grid grid-cols-1 md:grid-cols-2 gap-4">
            {children()}
        </div>
    }
}

#[component]
pub fn AppSubSection(
    heading: String,
    children: Children,
    position: SubSectionPosition,
) -> impl IntoView {
    let position_class = match position {
        SubSectionPosition::Left => "md:col-start-1 md:col-end-2",
        SubSectionPosition::Right => "md:col-start-2 md:col-end-3",
    };
    view! {
        <section class=format!("{} md:rounded-lg bg-table-section",position_class)>
            <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">{heading}</h1>
            {children()}
        </section>
    }
}

#[component]
pub fn AppSection(children: Children) -> impl IntoView {
    view! {
        <section class="md:col-start-2 md:col-end-3 md:rounded-lg bg-table-section mb-4">
            {children()}
        </section>
    }
}

#[component]
pub fn AppHeading(heading: String) -> impl IntoView {
    view! {
        <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">{heading}</h1>
    }
}

#[component]
pub fn Checkbox<F>(label: String, value: bool, handle_change: F) -> impl IntoView
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
                class="accent-granola-orange" />
            {label}
        </label>
    }
}

#[component]
pub fn URLCheckbox(label: String, url_param_key: String) -> impl IntoView {
    let query_params_map = use_query_map();
    let navigate = use_navigate();
    let location = use_location();

    let url_param_key_clone = url_param_key.clone(); // Clone url_param_key for use in the closure

    let initial_checkbox_value =
        move || query_params_map.with(|params| params.get(&url_param_key_clone).cloned());
    let (checkbox_value, set_checkbox_value) =
        create_signal(initial_checkbox_value().map_or(false, |i| i == "true"));

    create_effect(move |_| {
        let current_checkbox_value = checkbox_value.get();
        let pathname = location.pathname.get();
        let mut pm = query_params_map.get();
        pm.insert(
            url_param_key.to_string(),
            current_checkbox_value.to_string(),
        );

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
    });

    view! {
        <Checkbox label=label value=checkbox_value.get() handle_change=move |ev| {
            set_checkbox_value.update(|c| {
                logging::log!("new value is {}", event_target_checked(&ev));
                *c = event_target_checked(&ev)
            })
        }/>
    }
}

#[component]
pub fn SummaryItem(
    label: String,
    value: Option<String>,
    id: String,
    #[prop(optional)] imgsrc: String,
) -> impl IntoView {
    view! {
        <div class="h-24 w-96 p-4 max-w-full grid gap-2 grid-cols-[minmax(50px,50px)_1fr] bg-white rounded-md">
            <div class="cols-span-1 row-start-1 row-end-3 bg-light-granola-orange rounded-md flex justify-center items-center">
                <img src=imgsrc width=25 alt="logo"/>
            </div>
            <div class="col-start-2 col-end-3 font-bold text-xl flex justify-start items-end" id={id.clone()}>{
                {match value {
                    Some(str_val) => view! {<span>{str_val}</span>}.into_view(),
                    None => data_placeholder().into_view()
                }}
            }</div>
            <label class="row-start-2 col-start-2 col-end-3 text-sm text-slate-500 font-semibold flex justify-start items-start" for={id.clone()}>{label}</label>
        </div>
    }
}

#[component]
pub fn ErrorView<E: std::fmt::Debug>(err: E) -> impl IntoView {
    view! {
        <div class="error">
            { format!("Error: {:#?}", err) }
        </div>
    }
}

#[component]
pub fn NullView() -> impl IntoView {
    view! {}
}

#[component]
pub fn PageContainer(children: Children) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-[10%_80%_10%] bg-secondary-background rounded-t-3xl py-6 px-2 sm:px-0 grow">
            {children()}
        </div>
    }
}

#[component]
pub fn PreSectionContainer(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col md:flex-row md:px-[10vw] mb-4">
            {children()}
        </div>
    }
}

#[component]
pub fn NavLink<F>(nav_entry: NavEntry, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let location = use_location();
    let pathname = move || location.pathname.get();
    let href = nav_entry.href.clone();
    let base_link_class = "md:mx-1.5 my-6 mx-4 flex font-bold text-sm uppercase hover:text-granola-orange hover:underline hover:decoration-2 whitespace-nowrap";
    view! {
        <a on:click=on_click class={move || format!("{} {}",base_link_class, if pathname().contains(&href) {"text-granola-orange"} else {"text-white"})} href=nav_entry.href>
            {match nav_entry.icon {
                NavIcon::Home => view! { <HomeIcon /> },
                NavIcon::Blocks => view! { <BlockIcon /> },
                NavIcon::Transactions => view! { <TransactionIcon /> },
                NavIcon::More => view! { <MoreIcon /> },
                NavIcon::SNARKs => view! { <SnarkIcon /> },
                NavIcon::Staking => view! { <StakingIcon /> },
                NavIcon::Broadcast => view! { <BroadcastIcon /> },
            }}
            <div class="ml-0.5">{nav_entry.text}</div>
        </a>
    }
}

#[component]
pub fn TabLink(nav_entry: NavEntry) -> impl IntoView {
    let location = use_location();
    let pathname = move || location.pathname.get();
    let href = nav_entry.href.clone();
    let base_link_class = "mx-1 p-2 flex font-bold text-sm uppercase border-b border-b-2 whitespace-nowrap box-border";
    let active_state = "text-granola-orange border-granola-orange";
    let inactive_state = "text-white border-transparent hover:border-white";
    view! {
        <a class={move || format!("{} {}",base_link_class, if pathname().ends_with(&href) { active_state } else { inactive_state })} href=nav_entry.href>
            {match nav_entry.icon {
                NavIcon::Home => view! { <HomeIcon /> },
                NavIcon::Blocks => view! { <BlockIcon /> },
                NavIcon::Transactions => view! { <TransactionIcon /> },
                NavIcon::More => view! { <MoreIcon /> },
                NavIcon::SNARKs => view! { <SnarkIcon /> },
                NavIcon::Staking => view! { <StakingIcon /> },
                NavIcon::Broadcast => view! { <BroadcastIcon /> },
            }}
            <div class="ml-0.5">{nav_entry.text}</div>
        </a>
    }
}

#[component]
fn TabbedPage(tabs: Vec<NavEntry>) -> impl IntoView {
    view! {
        <PreSectionContainer>
            <ul class="flex w-full overflow-x-auto">
                {tabs.into_iter().map(|t| view!{
                    <li>
                        <TabLink nav_entry=t />
                    </li>
                }).collect::<Vec<_>>()}
            </ul>
        </PreSectionContainer>
        <Outlet />
    }
}

#[component]
pub fn DelegationTabbedPage() -> impl IntoView {
    let tabs = vec![
        NavEntry {
            href: "/broadcast/transaction".to_string(),
            text: "Transaction".to_string(),
            icon: NavIcon::Transactions,
            sub_entries: None,
        },
        NavEntry {
            href: "/broadcast/delegation".to_string(),
            text: "Delegation".to_string(),
            icon: NavIcon::Transactions,
            sub_entries: None,
        },
    ];
    view! { <TabbedPage tabs /> }
}
