use super::models::*;
use crate::{common::functions::*, icons::*};
use leptos::{html::AnyElement, web_sys::MouseEvent, *};

pub trait TableData {
    fn get_columns(&self) -> Vec<String>;
    fn get_rows(&self) -> Vec<Vec<HtmlElement<AnyElement>>>;
}

#[component]
pub fn Table<T>(data: T, #[prop(optional)] pagination: Option<Pagination>) -> impl IntoView
where
    T: TableData,
{
    let columns = data.get_columns();
    let rows = data.get_rows();
    let cell_padding_class = "first:pl-8 pl-2";
    let page_number_class = "text-md m-1 h-6 w-6 flex justify-center items-center font-semibold";

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
                    Some(pg) => {
                        let x_pages_around = x_surrounding_pages(pg.current_page, pg.total_pages());
                        let x_preceding_pages = &x_pages_around[0];
                        let x_following_pages = &x_pages_around[1];
                        view! {
                            <div class="flex flex-col md:grid md:grid-cols-3 min-h-12 bg-table-header-fill">
                                <span class="col-start-1 text-xs flex justify-center md:justify-start items-center font-bold pl-8 my-2">
                                    {format!("Showing {} to {} of {} records", pg.start_index(), pg.end_index(), pg.total_records)}
                                </span>
                                <span class="col-start-2 text-xs font-bold flex items-center justify-center my-2">
                                    <PaginationButton on_click=pg.prev_page disabled=x_preceding_pages.is_empty()>
                                        <ChevronLeft width=16/>
                                    </PaginationButton>
                                    {x_preceding_pages.iter()
                                        .map(|p| view! {
                                            <div class=page_number_class>{*p}</div>
                                        })
                                        .collect::<Vec<_>>()
                                    }
                                    <span class=format!("text-white rounded-md bg-granola-orange {}",page_number_class)>{pg.current_page}</span>
                                    {x_following_pages.iter()
                                        .map(|p| view! {
                                            <div class=page_number_class>{*p}</div>
                                        })
                                        .collect::<Vec<_>>()
                                    }
                                    <PaginationButton on_click=pg.next_page disabled=x_following_pages.is_empty()>
                                        <ChevronRight width=16/>
                                    </PaginationButton>
                                </span>
                            </div>
                        }.into_view()
                    },
                    None => view! { <NullView/> }
                }
            }
        }

    }
}

#[component]
fn PaginationButton(
    children: Children,
    on_click: Callback<MouseEvent>,
    disabled: bool,
) -> impl IntoView {
    let button_class_base = "font-semibold";
    let button_class = match disabled {
        true => format!(
            "{} {}",
            button_class_base, "text-slate-400 hover:cursor-not-allowed"
        ),
        false => format!(
            "{} {}",
            button_class_base, "hover:cursor-pointer hover:text-granola-orange hover:underline"
        ),
    };
    view! {
        <div class=button_class type="button" on:click=move |event: MouseEvent| {
            if disabled {
                return;
            }
            on_click.call(event)
        }>
            {children()}
        </div>
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
pub fn MainContainer(children: Children) -> impl IntoView {
    view! {
        <main class="grid grid-cols-1 md:grid-cols-[10%_80%_10%] bg-secondary-background rounded-t-3xl py-6 px-2 sm:px-0 grow">
            {children()}
        </main>
    }
}