use leptos::*;

pub enum SummaryItemKind {
    Str(String),
    Int64(u64),
    Int16(u16),
    Int32(u32),
    Float64(f64),
}

#[component]
pub fn SummaryItem(label: String, value: SummaryItemKind, id: String, #[prop(optional)] imgsrc: String) -> impl IntoView {
    view! {
        <div class="h-24 w-96 p-4 max-w-full grid gap-1 grid-cols-[100px_auto] bg-white rounded-md">
            <div class="cols-span-1 row-start-1 row-end-3 bg-light-granola-orange rounded-md flex justify-center items-center">
                <img src=imgsrc width=45 height=45 alt="logo"/>
            </div>
            <div class="col-start-2 col-end-3 font-bold text-xl flex justify-start items-end" id={id.clone()}>{
                match value {
                    SummaryItemKind::Str(s) => s,
                    SummaryItemKind::Int64(i) => i.to_string(),
                    SummaryItemKind::Int32(i) => i.to_string(),
                    SummaryItemKind::Int16(i) => i.to_string(),
                    SummaryItemKind::Float64(i) => format!("{:.2}", i).parse::<f64>().unwrap().to_string()
                }
            }</div>
            <label class="row-start-2 col-start-2 col-end-3 text-sm flex justify-start items-start" for={id.clone()}>{label}</label>
        </div>
    }
}
