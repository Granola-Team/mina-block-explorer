use leptos::*;

#[component]
pub fn Header() -> impl IntoView {

    let (open, set_open) = create_signal(false);
    let nav_items = vec![
        ("/summary", "Summary"),
        ("/blocks", "Blocks"),
        ("/transactions", "Transactions"),
        ("/snarks", "SNARKs"),
    ];

    let toggle = move |_| set_open.update(|value| *value = !*value);

    let base_class = "md:col-start-3 md:col-end-4 md:[all:unset] bg-main-background scale-y-0 transition-transform origin-top w-screen text-left absolute top-full left-0";
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
                        .map(|(link, title)| view! {
                            <li class="md:mb-0 mb-2 mx-2">
                                <a on:click=toggle class="font-bold text-sm uppercase text-white hover:text-granola-orange hover:underline hover:decoration-2" href=link>{title}</a>
                            </li>
                        }).collect::<Vec<_>>()}
                </ul>
            </nav>
            <label on:click=toggle for="nav-toggle" class="md:hidden absolute top-0 left-0 h-full ml-4 flex items-center ">
                <span class="relative rounded-lg block bg-white h-0.5 w-4 after:absolute after:rounded-lg after:block after:bg-white after:h-0.5 after:w-4 after:bottom-1 before:absolute before:roudned-sm before:block before:bg-white before:h-0.5 before:w-4 before:top-1"></span>
            </label>
        </header>
    }
}
