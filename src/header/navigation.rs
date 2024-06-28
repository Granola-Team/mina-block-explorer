use crate::common::{components::*, constants::*, models::*};
use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    let (open, set_open) = create_signal(false);
    let mut addr_entries = None;

    let mut txn_entries = Some(vec![
        NavEntry {
            href: "/commands/user".to_string(),
            text: "User Commands".to_string(),
            icon: NavIcon::Transactions,
            ..Default::default()
        },
        NavEntry {
            href: "/commands/internal".to_string(),
            text: "Internal Commands".to_string(),
            icon: NavIcon::Transactions,
            ..Default::default()
        },
    ]);

    if BERKELEY_FEATURES_ENABLED == "true" {
        if let Some(v) = txn_entries.as_mut() {
            v.push(NavEntry {
                href: "/commands/zk-txn".to_string(),
                text: "zkApp Commands".to_string(),
                icon: NavIcon::ZKApps,
                ..Default::default()
            })
        }

        addr_entries = Some(vec![
            NavEntry {
                href: "/addresses/accounts".to_string(),
                text: "Accounts".to_string(),
                icon: NavIcon::Accounts,
                ..Default::default()
            },
            NavEntry {
                href: "/addresses/tokens".to_string(),
                text: "Tokens".to_string(),
                icon: NavIcon::Tokens,
                ..Default::default()
            },
            NavEntry {
                href: "/addresses/zk-apps".to_string(),
                text: "zk-apps".to_string(),
                icon: NavIcon::ZKApps,
                ..Default::default()
            },
        ])
    }

    let nav_items = vec![
        NavEntry {
            href: "/blocks".to_string(),
            text: "Blocks".to_string(),
            icon: NavIcon::Blocks,
            ..Default::default()
        },
        NavEntry {
            href: "/commands/user".to_string(),
            text: "Transactions".to_string(),
            icon: NavIcon::Transactions,
            sub_entries: txn_entries,
            ..Default::default()
        },
        NavEntry {
            href: "/addresses/accounts".to_string(),
            text: "Addresses".to_string(),
            icon: NavIcon::Addresses,
            sub_entries: addr_entries,
            ..Default::default()
        },
        NavEntry {
            href: "/snarks".to_string(),
            text: "SNARKs".to_string(),
            icon: NavIcon::SNARKs,
            ..Default::default()
        },
        NavEntry {
            href: "/staking-ledgers".to_string(),
            text: "Staking".to_string(),
            icon: NavIcon::Staking,
            sub_entries: None,
            ..Default::default()
        },
        NavEntry {
            href: "#".to_string(),
            text: "More".to_string(),
            icon: NavIcon::More,
            disabled: false,
            sub_entries: Some(vec![
                NavEntry {
                    href: "/broadcast/transaction".to_string(),
                    text: "Send".to_string(),
                    icon: NavIcon::Send,
                    disabled: false,
                    sub_entries: None,
                    ..Default::default()
                },
                NavEntry {
                    href: "/analytics".to_string(),
                    text: "Analytics".to_string(),
                    icon: NavIcon::Analytics,
                    disabled: false,
                    sub_entries: None,
                    ..Default::default()
                },
            ]),
            ..Default::default()
        },
    ];

    let toggle = move |_| set_open.update(|value| *value = !*value);

    let base_class = "md:[all:unset] bg-main-background scale-y-0 transition-transform origin-top w-screen text-left absolute top-full left-0 overflow-hidden";
    let open_class = "scale-y-100";

    view! {
        <header class="pl-6 pr-6 pt-4 z-10 bg-main-background flex justify-center md:justify-between items-center fixed top-0 left-0 w-screen h-16">
            <span class="ribbon hidden md:block">Beta</span>
            <a href="/" class="flex items-center justify-start">
                <img src="/assets/img/logo.svg" width="45px" height="29px" alt="Minasearch"/>
                <span class="md:hidden lg:block sm:block ml-1 text-white font-bold text-xl">
                    Mina
                </span>
                <span class="md:hidden lg:block sm:block text-granola-orange font-bold text-xl">
                    Search
                </span>
            </a>
            <input id="nav-toggle" type="checkbox" class="hidden"/>
            <nav class=move || {
                format!("{} {}", base_class, if open.get() { open_class } else { "" })
            }>
                <ul class="md:flex md:justify-end m-0 p-0">
                    {nav_items
                        .into_iter()
                        .map(|nav_entry| {
                            let sub_entries = nav_entry.sub_entries.clone();
                            view! {
                                <li class="group relative">
                                    <NavLink nav_entry=nav_entry on_click=toggle/>
                                    {match sub_entries {
                                        Some(s_entries) => {
                                            view! {
                                                <ul class="md:px-2 md:hidden md:absolute md:top-0 md:left-0 md:bg-main-background md:shadow-md md:translate-y-16 md:-translate-x-1/2 group-hover:block">
                                                    {s_entries
                                                        .into_iter()
                                                        .map(|sub_entry| {
                                                            view! {
                                                                <li class="ml-4">
                                                                    <NavLink nav_entry=sub_entry on_click=toggle/>
                                                                </li>
                                                            }
                                                        })
                                                        .collect::<Vec<_>>()}
                                                </ul>
                                            }
                                                .into_view()
                                        }
                                        None => ().into_view(),
                                    }}

                                </li>
                            }
                        })
                        .collect::<Vec<_>>()}

                </ul>
            </nav>
            <label
                on:click=toggle
                for="nav-toggle"
                class="md:hidden absolute top-0 left-0 h-full ml-4 flex items-center "
            >
                <span class="relative rounded-lg block bg-white h-0.5 w-4 after:absolute after:rounded-lg after:block after:bg-white after:h-0.5 after:w-4 after:bottom-1 before:absolute before:roudned-sm before:block before:bg-white before:h-0.5 before:w-4 before:top-1"></span>
            </label>
        </header>
    }
}
