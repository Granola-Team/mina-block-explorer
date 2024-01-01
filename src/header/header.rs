use leptos::*;
use leptos_router::use_location;

enum Icon {
    Home,
    Blocks,
    Transactions,
    SNARKs
}

#[component]
pub fn Header() -> impl IntoView {

    let location = use_location();

    let (open, set_open) = create_signal(false);
    let nav_items = vec![
        ("/summary", "Summary", Icon::Home),
        ("/blocks", "Blocks", Icon::Blocks),
        ("/transactions", "Transactions", Icon::Transactions),
        ("/snarks", "SNARKs", Icon::SNARKs),
    ];

    let toggle = move |_| set_open.update(|value| *value = !*value);

    let base_class = "md:col-start-3 md:col-end-4 md:[all:unset] bg-main-background scale-y-0 transition-transform origin-top w-screen text-left absolute top-full left-0";
    let base_link_class = "flex font-bold text-sm uppercase hover:text-granola-orange hover:underline hover:decoration-2";
    let open_class = "scale-y-100";
    
    view! {
        <header class="z-10 bg-main-background flex justify-center items-center md:grid md:grid-cols-[10%_20%_60%_10%] md:col-start-2 md:col-end-3 fixed top-0 left-0 w-screen h-16">
            <div class="md:col-start-2 md:col-end-3 flex items-center justify-start">
                <img src="/img/logo.svg" width="45px" height="29px" alt="Minasearch" />
                <span class="ml-1 text-white font-bold text-xl">Mina</span><span class="text-granola-orange font-bold text-xl">Search</span>
            </div>
            <input id="nav-toggle" type="checkbox" class="hidden" />
            <nav class={move || format!("{} {}", base_class, if open.get() { open_class } else { "" })}>
                <ul class="md:flex md:justify-end m-0 p-0">
                    {nav_items.into_iter()
                        .map(|(link, title, icon)| {
                            let pathname = move || location.pathname.get();
                            view! {
                                <li class="md:mb-0 mb-2 mx-2">
                                    <a on:click=toggle class={move || format!("{} {}",base_link_class, if pathname() == link {"text-granola-orange"} else {"text-white"})} href=link>
                                        {match icon {
                                            Icon::Home => view! { <HomeIcon /> },
                                            Icon::Blocks => view! { <BlockIcon /> },
                                            Icon::Transactions => view! { <TransactionIcon /> },
                                            Icon::SNARKs => view! { <SnarkIcon /> },
                                        }}
                                        <div class="ml-1">{title}</div>
                                    </a>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                </ul>
            </nav>
            <label on:click=toggle for="nav-toggle" class="md:hidden absolute top-0 left-0 h-full ml-4 flex items-center ">
                <span class="relative rounded-lg block bg-white h-0.5 w-4 after:absolute after:rounded-lg after:block after:bg-white after:h-0.5 after:w-4 after:bottom-1 before:absolute before:roudned-sm before:block before:bg-white before:h-0.5 before:w-4 before:top-1"></span>
            </label>
        </header>
    }
}

#[component]
fn HomeIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" width="20">
            <path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
        </svg>
    }
}

#[component]
fn BlockIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" width="20">
            <path stroke-linecap="round" stroke-linejoin="round" d="m21 7.5-2.25-1.313M21 7.5v2.25m0-2.25-2.25 1.313M3 7.5l2.25-1.313M3 7.5l2.25 1.313M3 7.5v2.25m9 3 2.25-1.313M12 12.75l-2.25-1.313M12 12.75V15m0 6.75 2.25-1.313M12 21.75V19.5m0 2.25-2.25-1.313m0-16.875L12 2.25l2.25 1.313M21 14.25v2.25l-2.25 1.313m-13.5 0L3 16.5v-2.25" />
        </svg>
    }
}


#[component]
fn AccountIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" width="20">
            <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a2.25 2.25 0 0 0-2.25-2.25H15a3 3 0 1 1-6 0H5.25A2.25 2.25 0 0 0 3 12m18 0v6a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 18v-6m18 0V9M3 12V9m18 0a2.25 2.25 0 0 0-2.25-2.25H5.25A2.25 2.25 0 0 0 3 9m18 0V6a2.25 2.25 0 0 0-2.25-2.25H5.25A2.25 2.25 0 0 0 3 6v3" />
        </svg>
    }
}

#[component]
fn TransactionIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" width="20">
            <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
        </svg>
    }
}

#[component]
fn SnarkIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" width="20">
            <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 18 9 11.25l4.306 4.306a11.95 11.95 0 0 1 5.814-5.518l2.74-1.22m0 0-5.94-2.281m5.94 2.28-2.28 5.941" />
        </svg>
    }
}


#[component]
fn StakingIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" width="20">
            <path stroke-linecap="round" stroke-linejoin="round" d="m9 14.25 6-6m4.5-3.493V21.75l-3.75-1.5-3.75 1.5-3.75-1.5-3.75 1.5V4.757c0-1.108.806-2.057 1.907-2.185a48.507 48.507 0 0 1 11.186 0c1.1.128 1.907 1.077 1.907 2.185ZM9.75 9h.008v.008H9.75V9Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm4.125 4.5h.008v.008h-.008V13.5Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z" />
        </svg>
    }
}