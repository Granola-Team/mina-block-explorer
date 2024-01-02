use leptos::*;
use crate::styles::*;

#[component]
pub fn TableSection(section_heading: String, children: Children) -> impl IntoView {
    let breakout_child_styles = get_desktop_breakout_child_styles().to_styles_string();
    view! {
        <section class={format!("md:col-end-3 md:rounded-lg bg-table-section {}",breakout_child_styles)}>
            <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">{section_heading}</h1>    
            {children()}
        </section>
    }
}
