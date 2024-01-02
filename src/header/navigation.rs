use leptos::*;
use leptos_router::use_location;
use crate::icons::*;
use crate::styles::*;

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
    let breakout_container_styles = get_desktop_breakout_container_styles(Some(vec!["20%".to_string(),"60%".to_string()])).to_styles_string();
    let breakout_child_styles = get_desktop_breakout_child_styles().to_styles_string();
    
    view! {
        <header class={format!("z-10 bg-main-background flex justify-center items-center fixed top-0 left-0 w-screen h-16 {}",breakout_container_styles)}>
            <div class={format!("flex items-center justify-start {}",breakout_child_styles)}>
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
                                <li class="md:mb-0 mb-5 mx-2">
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
