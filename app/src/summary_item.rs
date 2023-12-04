use leptos::*;

pub enum SummaryItemKind {
    StrValue(String),
    Int64Value(u64),
    Int16Value(u16),
    Int32Value(u32),
    Float64Value(f64)
}

#[component]
pub fn SummaryItem(label: String, value: SummaryItemKind, id: String) -> impl IntoView {
    view! {
        <div class="flex">
            <label for={id.clone()}>{label}:</label>
            <div id={id.clone()}>{
                match value {
                    SummaryItemKind::StrValue(s) => s,
                    SummaryItemKind::Int64Value(i) => i.to_string(),
                    SummaryItemKind::Int32Value(i) => i.to_string(),
                    SummaryItemKind::Int16Value(i) => i.to_string(),
                    SummaryItemKind::Float64Value(i) => format!("{:.2}", i).parse::<f64>().unwrap().to_string()
                }
            }</div>
        </div>
    }
}
