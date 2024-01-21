use leptos::*;

#[component]
pub fn AppSection(children: Children) -> impl IntoView {
    view! {
        <section class="md:col-start-2 md:col-end-3 md:rounded-lg bg-table-section mb-4">
            {children()}
        </section>
    }
}

#[component]
pub fn AppHeading(heading: String) -> impl IntoView {
    view! {
        <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">{heading}</h1>
    }
}

pub enum SummaryItemKind {
    Str(String),
    Int64(u64),
    Int16(u16),
    Int32(u32),
    Float64(f64),
}

#[component]
pub fn SummaryItem(
    label: String,
    value: SummaryItemKind,
    id: String,
    #[prop(optional)] imgsrc: String,
) -> impl IntoView {
    view! {
        <div class="h-24 w-96 p-4 max-w-full grid gap-2 grid-cols-[minmax(50px,50px)_1fr] bg-white rounded-md">
            <div class="cols-span-1 row-start-1 row-end-3 bg-light-granola-orange rounded-md flex justify-center items-center">
                <img src=imgsrc width=25 alt="logo"/>
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
            <label class="row-start-2 col-start-2 col-end-3 text-sm text-slate-500 font-semibold flex justify-start items-start" for={id.clone()}>{label}</label>
        </div>
    }
}

#[component]
pub fn ErrorView<E: std::fmt::Debug>(err: E) -> impl IntoView {
    view! {
        <div class="error">
            { format!("Error: {:#?}", err) }
        </div>
    }
}

#[component]
pub fn NullView() -> impl IntoView {
    view! {}
}

#[component]
pub fn PageContainer(children: Children) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-[10%_80%_10%] bg-secondary-background rounded-t-3xl py-6 px-2 sm:px-0 grow">
            {children()}
        </div>
    }
}
