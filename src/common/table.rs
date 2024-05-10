use super::{components::*, functions::*, models::*};
use crate::{common::constants::*, icons::*};
use heck::ToKebabCase;
use leptos::{html::*, web_sys::MouseEvent, *};
use leptos_router::*;
use leptos_use::{use_debounce_fn_with_options, DebounceOptions};

pub trait TableData {
    fn get_columns(&self) -> Vec<String> {
        Vec::new()
    }
    fn get_rows(&self) -> Vec<Vec<HtmlElement<AnyElement>>>;
    fn get_exact_search_columns(&self) -> Vec<String> {
        Vec::new()
    }
}

#[derive(Clone)]
pub struct TableColumn {
    pub column: String,
    pub is_searchable: bool,
}

const INPUT_CLASS: &str = "w-5/6 mt-1 h-7 text-base text-sm font-normal font-mono p-2 rounded";
const PAGINATION_BUTTON_SIZE: &str = "h-7 w-7";
const BUTTON_CLASS_BASE: &str = "font-semibold flex justify-center items-center";
const PAGE_NUMBER_CLASS: &str = "page text-md m-1 flex justify-center items-center font-semibold";
const INACTIVE_PAGE_NUMBER_CLASS: &str =
    "hover:bg-slate-300 bg-transparent rounded-full cursor-pointer";
const CELL_PADDING_CLASS: &str = "first:pl-8 pl-2 last:pr-4";

#[component]
pub fn TableContainer(children: Children) -> impl IntoView {
    view! { <div class="@container w-full overflow-auto">{children()}</div> }
}

#[component]
pub fn Table(children: Children) -> impl IntoView {
    view! {
        <table class="font-mono md:rounded-b-lg w-full @xs:w-[175%] @md:w-[150%] @2xl:w-[125%] @7xl:w-full">
            {children()}
        </table>
    }
}

#[component]
pub fn DeprecatedTable<T>(
    data: T,
    #[prop(optional)] pagination: Option<Pagination>,
) -> impl IntoView
where
    T: TableData,
{
    let columns = data
        .get_columns()
        .iter()
        .map(|c| TableColumn {
            column: c.to_string(),
            is_searchable: data.get_exact_search_columns().contains(c),
        })
        .collect::<Vec<_>>();

    view! {
        <div class="@container w-full overflow-auto">
            <table class="font-mono md:rounded-b-lg w-full @xs:w-[175%] @md:w-[150%] @2xl:w-[125%] @7xl:w-full">
                <TableHeader columns=columns/>
                <TableRows data=data/>
            </table>
        </div>
        {pagination
            .map(|pag| view! { <Pagination pagination=pag/> })
            .unwrap_or_else(|| ().into_view())}
    }
}

#[component]
pub fn TableHeader(columns: Vec<TableColumn>) -> impl IntoView {
    view! {
        <tr class="h-12 bg-table-header-fill">
            {columns
                .iter()
                .map(|s| {
                    let id = "q-".to_string() + &s.column.as_str().to_kebab_case();
                    view! { <ColumnHeader id=id column=s.clone()/> }
                })
                .collect::<Vec<_>>()}
        </tr>
    }
}

#[component]
fn ColumnHeader(id: String, column: TableColumn) -> impl IntoView {
    let id_copy = id.clone();
    let (value, set_value) = create_query_signal::<String>(id);
    let input_element: NodeRef<html::Input> = create_node_ref();

    let update_value = use_debounce_fn_with_options(
        move || {
            let v = input_element
                .get()
                .expect("<input/> should be mounted")
                .value();
            if v.is_empty() {
                set_value.set(None);
            } else {
                set_value.set(Some(v));
            }
        },
        DEFAULT_USER_INPUT_DEBOUNCE_INTERNVAL,
        DebounceOptions::default(),
    );

    view! {
        <th class=format!(
            "{} text-table-header-text-color font-semibold uppercase text-xs text-left p-2 box-border",
            CELL_PADDING_CLASS,
        )>
            <div class="whitespace-nowrap">{column.column.clone()}</div>
            {if column.is_searchable {
                view! {
                    <input
                        value=value
                        on:input=move |_| {
                            update_value();
                        }

                        node_ref=input_element
                        class=INPUT_CLASS
                        id=id_copy
                    />
                }
                    .into_view()
            } else {
                view! { <div class=INPUT_CLASS></div> }.into_view()
            }}

        </th>
    }
}

#[component]
pub fn TableRows<T>(data: T) -> impl IntoView
where
    T: TableData,
{
    data.get_rows()
            .into_iter()
            .map(|row| {
                view! {
                    <tr class="h-12 bg-table-row-fill">
                        {row
                            .into_iter()
                            .map(|cell| {
                                let cell_ellipsis_class = "text-ellipsis overflow-hidden";
                                let cell_class = format!(
                                    "{} {} text-table-row-text-color font-medium text-sm text-left whitespace-nowrap max-w-40",
                                    CELL_PADDING_CLASS,
                                    cell_ellipsis_class,
                                );
                                view! { <td class=cell_class>{cell.into_view()}</td> }
                            })
                            .collect_view()}
                    </tr>
                }
            })
            .collect_view()
}

#[component]
pub fn Pagination(pagination: Pagination) -> impl IntoView {
    let x_pages_around = x_surrounding_pages(pagination.current_page, pagination.total_pages());
    let create_page_button = |page_num: usize, current_page: usize| {
        let is_current_page = page_num == current_page;
        let button_classes = format!(
            "{} {} {}",
            PAGINATION_BUTTON_SIZE,
            PAGE_NUMBER_CLASS,
            if is_current_page {
                "current-page text-white rounded-md bg-granola-orange"
            } else {
                INACTIVE_PAGE_NUMBER_CLASS
            }
        );

        vec![view! {
                <button
                    class=button_classes
                    on:click=move |_| if !is_current_page { pagination.set_current_page.update(|cp| *cp = page_num) }
                >{page_num}</button>
            }.into_view()]
    };

    let pg_clone = pagination.clone();

    view! {
        <div class="pagination-controls flex flex-col md:grid md:grid-cols-3 min-h-12 bg-table-header-fill">
            <span class="col-start-1 text-xs flex justify-center md:justify-start items-center font-bold pl-8 my-2">
                {format!(
                    "Showing {} to {} of {} records",
                    std::cmp::min(pagination.start_index() + 1, pagination.total_records),
                    std::cmp::min(pagination.end_index() + 1, pagination.total_records),
                    pagination.total_records,
                )}

            </span>
            <span class="button-container col-start-2 text-xs font-bold flex items-center justify-center my-2">
                <PaginationButton
                    class_id="go_to_first"
                    on_click=move |_| { pagination.set_current_page.update(|cp| *cp = 1) }

                    disabled=pagination.current_page == 1
                >
                    <ChevronDoubleLeft width=16/>
                </PaginationButton>
                <PaginationButton
                    class_id="go_to_prev"
                    on_click=move |_| {
                        pagination
                            .set_current_page
                            .update(|cp| *cp = pagination.current_page.saturating_sub(1))
                    }

                    disabled=pagination.current_page == 1
                >
                    <ChevronLeft width=16/>
                </PaginationButton>
                {x_pages_around[0]
                    .iter()
                    .flat_map(|&p| create_page_button(p, pagination.current_page))
                    .collect::<Vec<_>>()}
                {create_page_button(pagination.current_page, pagination.current_page)}
                {x_pages_around[1]
                    .iter()
                    .flat_map(|&p| create_page_button(p, pagination.current_page))
                    .collect::<Vec<_>>()}
                <PaginationButton
                    class_id="go_to_next"
                    on_click=move |_| {
                        pagination.set_current_page.update(|cp| *cp = pagination.current_page + 1)
                    }

                    disabled=pagination.current_page == pagination.total_pages()
                >
                    <ChevronRight width=16/>
                </PaginationButton>
                <PaginationButton
                    class_id="go_to_last"
                    on_click=move |_| {
                        pagination.set_current_page.update(|cp| *cp = pg_clone.total_pages())
                    }

                    disabled=pagination.current_page == pagination.total_pages()
                >
                    <ChevronDoubleRight width=16/>
                </PaginationButton>
            </span>
        </div>
    }.into_view()
}

#[component]
fn PaginationButton(
    #[prop(into)] class_id: String,
    children: Children,
    #[prop(into)] on_click: Callback<MouseEvent>,
    disabled: bool,
) -> impl IntoView {
    let button_class = if disabled {
        format!(
            "{} {} text-slate-400 hover:cursor-not-allowed",
            PAGINATION_BUTTON_SIZE, BUTTON_CLASS_BASE
        )
    } else {
        format!("{} {} hover:cursor-pointer hover:text-granola-orange hover:underline rounded-full bg-transparent hover:bg-slate-300", PAGINATION_BUTTON_SIZE, BUTTON_CLASS_BASE)
    };

    view! {
        <button
            class=class_id + " " + &button_class
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
    #[prop(optional, into)] additional_info: String,
    controls: F,
) -> impl IntoView
where
    E: IntoView,
    F: Fn() -> E + 'static,
{
    view! {
        <AppSection>
            <span class="w-full flex justify-between flex-wrap">
                <AppHeading heading=section_heading/>
                {if !additional_info.is_empty() {
                    view! {
                        <div class="additional-info pl-8 pr-4 h-16 grow flex justify-start md:justify-center items-center text-slate-400 text-normal text-sm italic font-thin">
                            {additional_info}
                        </div>
                    }
                        .into_view()
                } else {
                    ().into_view()
                }}

                <div class="grow md:grow-0 h-16 flex justify-end items-center pr-4">
                    {controls()}
                </div>
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

#[derive(Clone)]
pub struct LoadingPlaceholder;

impl TableData for Vec<Vec<LoadingPlaceholder>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|row| row.iter().map(|_| data_placeholder()).collect())
            .collect()
    }
}

pub struct DeprecatedLoadingPlaceholder;

impl TableData for DeprecatedLoadingPlaceholder {
    fn get_columns(&self) -> Vec<String> {
        vec![String::new().clone(); 5]
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        [[""; 5]; 5]
            .iter()
            .map(|o| o.iter().map(|_| data_placeholder()).collect::<Vec<_>>())
            .collect()
    }
}
