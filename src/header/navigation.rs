use crate::common::components::*;
use crate::common::models::*;
use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    let (open, set_open) = create_signal(false);
    let nav_items = vec![
        NavEntry {
            href: "/blocks".to_string(),
            text: "Blocks".to_string(),
            icon: NavIcon::Blocks,
            sub_entries: None,
        },
        NavEntry {
            href: "/transactions".to_string(),
            text: "Transactions".to_string(),
            icon: NavIcon::Transactions,
            sub_entries: None,
        },
        NavEntry {
            href: "/accounts".to_string(),
            text: "Accounts".to_string(),
            icon: NavIcon::Accounts,
            sub_entries: None,
        },
        NavEntry {
            href: "/snarks".to_string(),
            text: "SNARKs".to_string(),
            icon: NavIcon::SNARKs,
            sub_entries: None,
        },
        NavEntry {
            href: "/stakes".to_string(),
            text: "Staking".to_string(),
            icon: NavIcon::Staking,
            sub_entries: None,
        },
        NavEntry {
            href: "#".to_string(),
            text: "More".to_string(),
            icon: NavIcon::More,
            sub_entries: Some(vec![NavEntry {
                href: "/broadcast".to_string(),
                text: "Broadcast".to_string(),
                icon: NavIcon::Broadcast,
                sub_entries: None,
            }]),
        },
    ];

    let toggle = move |_| set_open.update(|value| *value = !*value);

    let base_class = "md:col-start-3 md:col-end-4 md:[all:unset] bg-main-background scale-y-0 transition-transform origin-top w-screen text-left absolute top-full left-0";
    let open_class = "scale-y-100";

    view! {
        <header class="z-10 bg-main-background flex justify-center items-center md:grid md:grid-cols-[10%_20%_60%_10%] md:col-start-2 md:col-end-3 fixed top-0 left-0 w-screen h-16">
            <a href="/" class="md:col-start-2 md:col-end-3 flex items-center justify-start">
                <img src="/img/logo.svg" width="45px" height="29px" alt="Minasearch" />
                <span class="ml-1 text-white font-bold text-xl">Mina</span><span class="text-granola-orange font-bold text-xl">Search</span>
            </a>
            <input id="nav-toggle" type="checkbox" class="hidden" />
            <nav class={move || format!("{} {}", base_class, if open.get() { open_class } else { "" })}>
                <ul class="md:flex md:justify-end m-0 p-0">
                    {nav_items.into_iter()
                        .map(|nav_entry| {
                            let sub_entries = nav_entry.sub_entries.clone();
                            view! {
                                <li class="group relative">
                                    <NavLink nav_entry=nav_entry on_click=toggle />
                                    { match sub_entries {
                                        Some(s_entries) => view! {
                                            <ul class="md:px-2 md:hidden md:absolute md:top-0 md:left-0 md:bg-main-background md:shadow-md md:translate-y-16 md:-translate-x-2/4 group-hover:block">
                                                {s_entries.into_iter()
                                                    .map(|sub_entry| view! {
                                                        <li class="ml-4">
                                                            <NavLink nav_entry=sub_entry on_click=toggle />
                                                        </li>
                                                    }).collect::<Vec<_>>()}
                                            </ul>
                                        }.into_view(),
                                        None => view! { <NullView /> },
                                    }}
                                </li>
                            }
                        }).collect::<Vec<_>>()
                    }
                </ul>
            </nav>
            <label on:click=toggle for="nav-toggle" class="md:hidden absolute top-0 left-0 h-full ml-4 flex items-center ">
                <span class="relative rounded-lg block bg-white h-0.5 w-4 after:absolute after:rounded-lg after:block after:bg-white after:h-0.5 after:w-4 after:bottom-1 before:absolute before:roudned-sm before:block before:bg-white before:h-0.5 before:w-4 before:top-1"></span>
            </label>
        </header>
    }
}
