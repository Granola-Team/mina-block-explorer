use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    let nav_items = vec![
        ("/summary", "Summary"),
        ("/blocks", "Blocks"),
        ("/transactions", "Transactions"),
        ("/snarks", "SNARKs"),
    ];
    view! {
        <input id="nav-toggle" type="checkbox" class="hidden peer/menu" />
        <nav class="md:col-start-3 md:col-end-4 md:[all:unset] bg-main-background peer-checked/menu:scale-y-100 scale-y-0 transition-transform origin-top w-screen text-left absolute top-full left-0">
            <ul class="md:flex md:justify-end m-0 p-0">
            {nav_items.into_iter()
                .map(|(link,title)| view! { <li class="md:mb-0 mb-2 mx-2"><a class="font-bold text-sm uppercase text-white hover:text-granola-orange hover:underline hover:decoration-2" href=link>{title}</a></li>})
                .collect::<Vec<_>>()}
            </ul>
        </nav>
        <label for="nav-toggle" class="md:hidden absolute top-0 left-0 h-full ml-4 flex items-center ">
            <span class="relative rounded-lg block bg-white h-0.5 w-4 after:absolute after:rounded-lg after:block after:bg-white after:h-0.5 after:w-4 after:bottom-1 before:absolute before:roudned-sm before:block before:bg-white before:h-0.5 before:w-4 before:top-1"></span>
        </label>
    }
}
