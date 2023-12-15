use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    let nav_items = vec![
        ("/summary", "Summary"),
        ("/blocks", "Blocks"),
        ("/accounts", "Accounts"),
        ("/transactions", "Transactions"),
        ("/snarks", "SNARKs"),
    ];
    view! {
        <input id="nav-toggle" type="checkbox" class="hidden peer/menu" />
        <nav class="md:col-start-3 md:col-end-4 md:[all:unset] bg-white peer-checked/menu:scale-y-100 scale-y-0 transition-transform origin-top w-screen text-left absolute top-full left-0">
            <ul class="md:flex md:justify-end m-0 p-0">
            {nav_items.into_iter()
                .map(|(link,title)| view! { <li class="md:mb-0 mb-2 mx-4"><a class="uppercase" href=link>{title}</a></li>})
                .collect::<Vec<_>>()}
            </ul>
        </nav>
        <label for="nav-toggle" class="md:hidden absolute top-0 left-0 h-full ml-4 flex items-center ">
            <span class="relative rounded-sm block bg-black h-0.5 w-8 after:absolute after:rounded-sm after:block after:bg-black after:h-0.5 after:w-8 after:bottom-1 before:absolute before:roudned-sm before:block before:bg-black before:h-0.5 before:w-8 before:top-1"></span>
        </label>
    }
}
