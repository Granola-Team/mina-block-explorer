use leptos::web_sys::MouseEvent;
use leptos::{html::*, *};

use super::components::*;
use super::functions::*;
use super::models::*;
use crate::icons::*;

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
        <AppSection>
            <AppHeading heading=section_heading />
            {children()}
        </AppSection>
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
