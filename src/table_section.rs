use leptos::*;

#[component]
pub fn TableSection(section_heading: String, children: Children) -> impl IntoView {
    view! {
        <section class="md:rounded-lg bg-table-section">
            <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">{section_heading}</h1>    
            {children()}
        </section>
    }
}
