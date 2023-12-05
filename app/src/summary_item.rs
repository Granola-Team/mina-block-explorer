use leptos::*;

pub enum SummaryItemKind {
    Str(String),
    Int64(u64),
    Int16(u16),
    Int32(u32),
    Float64(f64)
}

#[component]
pub fn SummaryItem(label: String, value: SummaryItemKind, id: String) -> impl IntoView {
    view! {
        <div class="flex">
            <label for={id.clone()}>{label}:</label>
            <div id={id.clone()}>{
                match value {
                    SummaryItemKind::Str(s) => s,
                    SummaryItemKind::Int64(i) => i.to_string(),
                    SummaryItemKind::Int32(i) => i.to_string(),
                    SummaryItemKind::Int16(i) => i.to_string(),
                    SummaryItemKind::Float64(i) => format!("{:.2}", i).parse::<f64>().unwrap().to_string()
                }
            }</div>
        </div>
    }
}
