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
        <section class="bg-white dark:bg-gray-900">
            <div class="py-4 px-2 mx-auto max-w-screen-md text-center md:py-8 md:px-4">
                <svg
                    class="mx-auto mb-4 w-10 h-10 text-gray-400"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 512 512"
                >
                    <path
                        fill="currentColor"
                        d="M331.8 224.1c28.29 0 54.88 10.99 74.86 30.97l19.59 19.59c40.01-17.74 71.25-53.3 81.62-96.65c5.725-23.92 5.34-47.08 .2148-68.4c-2.613-10.88-16.43-14.51-24.34-6.604l-68.9 68.9h-75.6V97.2l68.9-68.9c7.912-7.912 4.275-21.73-6.604-24.34c-21.32-5.125-44.48-5.51-68.4 .2148c-55.3 13.23-98.39 60.22-107.2 116.4C224.5 128.9 224.2 137 224.3 145l82.78 82.86C315.2 225.1 323.5 224.1 331.8 224.1zM384 278.6c-23.16-23.16-57.57-27.57-85.39-13.9L191.1 158L191.1 95.99l-127.1-95.99L0 63.1l96 127.1l62.04 .0077l106.7 106.6c-13.67 27.82-9.251 62.23 13.91 85.39l117 117.1c14.62 14.5 38.21 14.5 52.71-.0016l52.75-52.75c14.5-14.5 14.5-38.08-.0016-52.71L384 278.6zM227.9 307L168.7 247.9l-148.9 148.9c-26.37 26.37-26.37 69.08 0 95.45C32.96 505.4 50.21 512 67.5 512s34.54-6.592 47.72-19.78l119.1-119.1C225.5 352.3 222.6 329.4 227.9 307zM64 472c-13.25 0-24-10.75-24-24c0-13.26 10.75-24 24-24S88 434.7 88 448C88 461.3 77.25 472 64 472z"
                    ></path>
                </svg>
                <h1 class="mb-4 text-2xl font-bold tracking-tight leading-none text-gray-900 md:text-3xl xl:text-4xl dark:text-white">
                    Early Access
                </h1>
                <p class="font-light text-gray-500 md:text-lg xl:text-xl dark:text-gray-400">
                    Explore and join us in refining the experience.
                </p>
            </div>
        </section>
    }
}
