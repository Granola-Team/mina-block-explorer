use crate::icons::*;
use leptos::{html::AnyElement, *};

pub trait TableData {
    fn get_columns(&self) -> Vec<String>;
    fn get_rows(&self) -> Vec<Vec<HtmlElement<AnyElement>>>;
}

#[derive(Clone)]
pub struct Pagination {
    pub current_page: i32,
    pub records_per_page: i32,
    pub total_records: i32,
    pub next_page: fn(),
    pub prev_page: fn(),
}

impl Pagination {
    fn start_index(&self) -> i32 {
        self.current_page * self.records_per_page - self.records_per_page + 1
    }

    fn end_index(&self) -> i32 {
        self.current_page * self.records_per_page
    }

    // fn total_pages(&self) -> i32 {
    //     self.total_records / self.records_per_page + (self.total_records % self.records_per_page).signum()
    // }
}

#[test]
fn test_indexes_first_page() {
    fn noop() {}
    let pd = Pagination {
        current_page: 1,
        records_per_page: 15,
        total_records: 90,
        next_page: noop,
        prev_page: noop,
    };
    assert_eq!(pd.start_index(), 1);
    assert_eq!(pd.end_index(), 15)
}

#[test]
fn test_indexes_second_page() {
    fn noop() {}
    let pd = Pagination {
        current_page: 2,
        records_per_page: 15,
        total_records: 90,
        next_page: noop,
        prev_page: noop,
    };
    assert_eq!(pd.start_index(), 16);
    assert_eq!(pd.end_index(), 30)
}

// #[test]
// fn test_total_pages() {
//     fn noop() {}
//     let pd = Pagination {
//         current_page: 2,
//         records_per_page: 15,
//         total_records: 90,
//         next_page: noop,
//         prev_page: noop,
//     };
//     assert_eq!(pd.total_pages(), 6);
//     let pd = Pagination {
//         current_page: 2,
//         records_per_page: 15,
//         total_records: 91,
//         next_page: noop,
//         prev_page: noop,
//     };
//     assert_eq!(pd.total_pages(), 7);
// }

#[component]
pub fn Table<T>(data: T, #[prop(optional)] pagination: Option<Pagination>) -> impl IntoView
where
    T: TableData,
{
    let columns = data.get_columns();
    let rows = data.get_rows();
    let cell_padding_class = "first:pl-8 pl-2";

    view! {
        <div class="@container w-full overflow-auto">
            <table class="md:rounded-b-lg table-fixed w-full @xs:w-[300%] @md:w-[200%] @2xl:w-[150%] @7xl:w-full">
            <tr class="h-12 bg-table-header-fill">
                {columns.into_iter()
                    .map(|s| view! { <th class={format!("{} text-table-header-text-color font-semibold uppercase text-xs text-left", cell_padding_class)}>{s}</th>})
                    .collect::<Vec<_>>()}
            </tr>
            {rows.into_iter()
                .map(|row| view! {
                    <tr class="h-12 bg-table-row-fill">
                        {
                            row.iter().map(|cell| {

                                let cell_ellipsis_class = "w-full text-ellipsis overflow-hidden";
                                let cell_class = format!("{} {} first:pl-8 pl-2 text-table-row-text-color font-medium text-sm text-left whitespace-nowrap", cell_padding_class, cell_ellipsis_class);
                                view! {
                                    <td class=cell_class>{cell.clone().into_view()}</td>
                                }

                            }).collect::<Vec<_>>()

                        }
                    </tr>
                })
                .collect::<Vec<_>>()}
            </table>
        </div>
        {
            let page_data_clone = pagination.clone();
            move || {
                let page_data_inner = page_data_clone.clone();
                match page_data_inner {
                    Some(pg) => view! {
                        <div class="grid grid-cols-3 h-12 bg-table-header-fill">
                            <span class="col-start-1 text-xs flex items-center font-bold pl-8">
                                {format!("Showing {} to {} of {} records", pg.start_index(), pg.end_index(), pg.total_records)}
                            </span>
                            <span class="col-start-2 text-xs font-bold flex items-center justify-center">
                                <button on:click=move |_| (pg.prev_page)()>"<< Previous Page"</button>
                                <span class="text-md m-4 underline">{pg.current_page}</span>
                                <button on:click=move |_| (pg.next_page)()>"Next Page>>"</button>
                            </span>
                        </div>
                    },
                    None => view! { <div/> }
                }
            }
        }

    }
}

#[component]
pub fn EmptyTable(message: String) -> impl IntoView {
    view! {
        <div class="flex text-base text-slate-400 items-center justify-center p-8">
            <NoIcon />
            <span class="text-sm">{message}</span>
        </div>
    }
}

#[component]
pub fn TableSection(section_heading: String, children: Children) -> impl IntoView {
    view! {
        <section class="md:col-start-2 md:col-end-3 md:rounded-lg bg-table-section mb-4">
            <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">{section_heading}</h1>
            {children()}
        </section>
    }
}

#[component]
pub fn TableLink(href: String, text: String, children: Children) -> impl IntoView {
    view! {
        <div class="w-full bg-inherit flex justify-center items-center py-3">
            <a href={href} class="font-bold uppercase text-sm flex justify-center align-center hover:underline hover:text-granola-orange">
                {children()}
                <span class="mx-1">{text}</span>
                <ChevronRight />
            </a>
        </div>
    }.into_view()
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
