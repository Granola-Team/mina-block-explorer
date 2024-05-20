use super::{components::*, functions::*, models::*};
use crate::{common::constants::*, icons::*};
use heck::ToKebabCase;
use leptos::{html::*, *};
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
pub fn DeprecatedTable<T>(data: T) -> impl IntoView
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
    #[prop(optional)] metadata: Option<TableMetadata>,
    controls: F,
) -> impl IntoView
where
    E: IntoView,
    F: Fn() -> E + 'static,
{
    let BASE_META_CLASS = "h-16 grow flex justify-start md:justify-center items-center text-slate-400 text-normal text-xs";
    view! {
        <AppSection>
            <span class="w-full flex justify-between flex-wrap">
                <div class="flex justify-start items-baseline flex-wrap">
                    <AppHeading heading=section_heading/>
                    {move || match metadata.clone() {
                        Some(meta) if meta.displayed_records >= TABLE_ROW_LIMIT => {
                            view! {
                                <div class="metadata pl-4 ".to_string()
                                    + BASE_META_CLASS>
                                    {format!(
                                        "Showing {} of {}",
                                        meta.displayed_records,
                                        meta.total_records,
                                    )}

                                </div>
                            }
                                .into_view()
                        }
                        Some(meta) if meta.displayed_records < TABLE_ROW_LIMIT => {
                            view! {
                                <div class="metadata pl-4 ".to_string()
                                    + BASE_META_CLASS>"Showing all"</div>
                            }
                                .into_view()
                        }
                        _ => ().into_view(),
                    }}

                    {if !additional_info.is_empty() {
                        view! {
                            <div class="px-4 text-normal text-slate-400">"|"</div>
                            <div class="additional-info ".to_string()
                                + BASE_META_CLASS>{additional_info}</div>
                        }
                            .into_view()
                    } else {
                        ().into_view()
                    }}

                </div>
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
