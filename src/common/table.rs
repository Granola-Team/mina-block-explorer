use super::{components::*, functions::*, models::*};
use crate::icons::*;
use leptos::{html::*, web_sys::MouseEvent, *};

pub trait TableData {
    fn get_columns(&self) -> Vec<String>;
    fn get_rows(&self) -> Vec<Vec<HtmlElement<AnyElement>>>;
}

const BUTTON_CLASS_BASE: &str = "font-semibold h-6 w-6 flex justify-center items-center";
const PAGE_NUMBER_CLASS: &str =
    "page text-md m-1 h-6 w-6 flex justify-center items-center font-semibold";
const INACTIVE_PAGE_NUMBER_CLASS: &str =
    "hover:bg-slate-300 bg-transparent rounded-full cursor-pointer";
const CELL_PADDING_CLASS: &str = "first:pl-8 pl-2 last:pr-4";

#[component]
pub fn Table<T>(data: T, #[prop(optional)] pagination: Option<Pagination>) -> impl IntoView
where
    T: TableData,
{
    let columns = data.get_columns();
    let rows = data.get_rows();

    view! {
        <div class="@container w-full overflow-auto">
            <table class="font-mono md:rounded-b-lg w-full @xs:w-[175%] @md:w-[150%] @2xl:w-[125%] @7xl:w-full">
                {generate_table_header(&columns)} {generate_table_rows(&rows)}
            </table>
        </div>
        {generate_pagination(pagination)}
    }
}

fn generate_table_header(columns: &[String]) -> impl IntoView {
    view! {
        <tr class="h-12 bg-table-header-fill">
            {columns
                .iter()
                .map(|s| {
                    view! {
                        <th class=format!(
                            "{} text-table-header-text-color font-semibold uppercase text-xs text-left",
                            CELL_PADDING_CLASS,
                        )>{s}</th>
                    }
                })
                .collect::<Vec<_>>()}
        </tr>
    }
}

fn generate_table_rows(rows: &[Vec<HtmlElement<AnyElement>>]) -> impl IntoView {
    view! {
        {rows
            .iter()
            .map(|row| {
                view! {
                    <tr class="h-12 bg-table-row-fill">
                        {row
                            .iter()
                            .map(|cell| {
                                let cell_ellipsis_class = "text-ellipsis overflow-hidden";
                                let cell_class = format!(
                                    "{} {} text-table-row-text-color font-medium text-sm text-left whitespace-nowrap max-w-40",
                                    CELL_PADDING_CLASS,
                                    cell_ellipsis_class,
                                );
                                view! { <td class=cell_class>{cell.clone().into_view()}</td> }
                            })
                            .collect::<Vec<_>>()}
                    </tr>
                }
            })
            .collect::<Vec<_>>()}
    }
}

fn generate_pagination(pagination: Option<Pagination>) -> impl IntoView {
    pagination.map(|pg| {
        let x_pages_around = x_surrounding_pages(pg.current_page, pg.total_pages());
        let create_page_button = |page_num: usize, current_page: usize| {
            let is_current_page = page_num == current_page;
            let button_classes = format!(
                "{} {}", 
                PAGE_NUMBER_CLASS,
                if is_current_page { "current-page text-white rounded-md bg-granola-orange" } else { INACTIVE_PAGE_NUMBER_CLASS }
            );

            vec![view! {
                <button
                    class=button_classes
                    on:click=move |_| if !is_current_page { pg.set_current_page.update(|cp| *cp = page_num) }
                >{page_num}</button>
            }.into_view()]
        };

        view! {
            <div class="pagination-controls flex flex-col md:grid md:grid-cols-3 min-h-12 bg-table-header-fill">
                <span class="col-start-1 text-xs flex justify-center md:justify-start items-center font-bold pl-8 my-2">
                    {format!(
                        "Showing {} to {} of {} records",
                        pg.start_index(),
                        std::cmp::min(pg.end_index(), pg.total_records),
                        pg.total_records,
                    )}

                </span>
                <span class="button-container col-start-2 text-xs font-bold flex items-center justify-center my-2">
                    <PaginationButton
                        on_click=move |_| {
                            pg.set_current_page.update(|cp| *cp = pg.current_page.saturating_sub(1))
                        }

                        disabled=pg.current_page == 1
                    >
                        <ChevronLeft width=16/>
                    </PaginationButton>
                    {x_pages_around[0]
                        .iter()
                        .flat_map(|&p| create_page_button(p, pg.current_page))
                        .collect::<Vec<_>>()}
                    {create_page_button(pg.current_page, pg.current_page)}
                    {x_pages_around[1]
                        .iter()
                        .flat_map(|&p| create_page_button(p, pg.current_page))
                        .collect::<Vec<_>>()}
                    <PaginationButton
                        on_click=move |_| pg.set_current_page.update(|cp| *cp = pg.current_page + 1)
                        disabled=pg.current_page == pg.total_pages()
                    >
                        <ChevronRight width=16/>
                    </PaginationButton>
                </span>
            </div>
        }.into_view()
    }).unwrap_or_else(|| ().into_view())
}

#[component]
fn PaginationButton(
    children: Children,
    #[prop(into)] on_click: Callback<MouseEvent>,
    disabled: bool,
) -> impl IntoView {
    let button_class = if disabled {
        format!(
            "{} text-slate-400 hover:cursor-not-allowed",
            BUTTON_CLASS_BASE
        )
    } else {
        format!("{} hover:cursor-pointer hover:text-granola-orange hover:underline rounded-full bg-transparent hover:bg-slate-300", BUTTON_CLASS_BASE)
    };

    view! {
        <button
            class=button_class
            disabled=disabled
            type="button"
            on:click=move |event: MouseEvent| {
                if !disabled {
                    on_click.call(event)
                }
            }
        >

            {children()}
        </button>
    }
}

#[component]
pub fn EmptyTable(#[prop(into)] message: String) -> impl IntoView {
    view! {
        <div class="flex text-base text-slate-400 items-center justify-center p-8">
            <NoIcon/>
            <span class="text-sm">{message}</span>
        </div>
    }
}

#[component]
pub fn TableSection<E, F>(
    #[prop(into)] section_heading: String,
    children: Children,
    controls: F,
) -> impl IntoView
where
    E: IntoView,
    F: Fn() -> E + 'static,
{
    view! {
        <AppSection>
            <span class="w-full flex justify-between">
                <AppHeading heading=section_heading/>
                <div class="self-stretch flex items-center pr-4">{controls()}</div>
            </span>
            {children()}
        </AppSection>
    }
}

#[component]
pub fn TableLink(
    #[prop(into)] href: String,
    #[prop(into)] text: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="w-full bg-inherit flex justify-center items-center py-3">
            <a
                href=href
                class="font-bold uppercase text-sm flex justify-center align-center hover:underline hover:text-granola-orange"
            >
                {children()}
                <span class="mx-1">{text}</span>
                <ChevronRight/>
            </a>
        </div>
    }.into_view()
}

pub struct LoadingPlaceholder;

impl TableData for LoadingPlaceholder {
    fn get_columns(&self) -> Vec<String> {
        vec![String::new().clone(); 5]
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        vec![vec![String::new().clone(); 5]; 10]
            .iter()
            .map(|o| o.iter().map(|_| data_placeholder()).collect::<Vec<_>>())
            .collect()
    }
}
