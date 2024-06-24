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
pub enum TableSortDirection {
    // ASC,
    Desc,
}

#[derive(Clone)]
pub struct TableColumn {
    pub column: String,
    pub is_searchable: bool,
    pub sort_direction: Option<TableSortDirection>,
    pub width: Option<String>,
    pub html_input_type: String,
}

impl Default for TableColumn {
    fn default() -> Self {
        TableColumn {
            column: String::new(),
            is_searchable: false,
            sort_direction: None,
            width: None,
            html_input_type: "text".to_string(),
        }
    }
}

const INPUT_CLASS: &str = "w-5/6 mt-1 h-7 text-base text-sm font-normal font-mono p-2 rounded";
const CELL_PADDING_CLASS: &str = "first:pl-8 pl-2 last:pr-4";

#[component]
pub fn TableContainer(children: Children) -> impl IntoView {
    view! { <div class="@container w-full overflow-auto xl:overflow-visible">{children()}</div> }
}

#[component]
pub fn Table(children: Children) -> impl IntoView {
    view! {
        <table class="xl:relative font-mono md:rounded-b-lg w-full @xs:w-[400%] @md:w-[300%] @2xl:w-[200%] xl:w-full">
            {children()}
        </table>
    }
}

#[component]
pub fn TableSectionTemplate<T, F, E>(
    table_columns: Vec<TableColumn>,
    data_sig: ReadSignal<Option<T>>,
    is_loading: Signal<bool>,
    #[prop(optional)] metadata: Option<Signal<Option<TableMetadata>>>,
    #[prop(into)] section_heading: String,
    #[prop(optional, into)] additional_info: View,
    controls: F,
) -> impl IntoView
where
    E: IntoView,
    F: Fn() -> E + 'static,
    T: TableData + Clone + 'static,
{
    let table_cols_length = table_columns.len();

    view! {
        <TableSection
            metadata=metadata.unwrap_or_default()
            section_heading
            controls
            additional_info
        >
            <TableContainer>
                <Table>
                    <ColGroup columns=table_columns.clone()/>
                    <TableHeader columns=table_columns/>

                    {move || {
                        if is_loading.get() {
                            view! {
                                <TableRows data=vec![
                                    vec![LoadingPlaceholder; table_cols_length];
                                    TABLE_ROW_LIMIT.try_into().unwrap_or_default()
                                ]/>
                            }
                        } else {
                            match data_sig.get() {
                                Some(data) => {
                                    view! { <TableRows data=data/> }
                                }
                                None => ().into_view(),
                            }
                        }
                    }}

                </Table>
                {move || {
                    if let Some(data) = data_sig.get() {
                        if data.get_rows().is_empty() {
                            view! { <EmptyTable message="No data for this view"/> }
                        } else {
                            ().into_view()
                        }
                    } else {
                        view! { <EmptyTable message="No data for this view"/> }
                    }
                }}

            </TableContainer>
        </TableSection>
    }
}

#[component]
pub fn ColGroup(columns: Vec<TableColumn>) -> impl IntoView {
    view! {
        {columns
            .iter()
            .map(|c| {
                c.width
                    .as_ref()
                    .map(|w| {
                        view! { <col width=w/> }
                    })
            })
            .collect_view()}
    }
}

#[component]
pub fn TableHeader(columns: Vec<TableColumn>) -> impl IntoView {
    view! {
        <tr>
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

const ICON_CLASS: &str = "cursor-pointer pl-1 flex justify-center items-center ";

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
        <th class="h-12 bg-table-header-fill xl:sticky xl:top-16 z-20 text-table-header-text-color font-semibold uppercase text-xs text-left p-2 box-border "
            .to_string() + CELL_PADDING_CLASS>
            <div class="whitespace-nowrap flex">
                <span class="pr-1">{column.column.clone()}</span>
                {match column.sort_direction {
                    Some(TableSortDirection::Desc) => {
                        view! {
                            <span class=ICON_CLASS>
                                <DownArrow width=12/>
                            </span>
                        }
                            .into_view()
                    }
                    None => ().into_view(),
                }}

            </div>
            {if column.is_searchable {
                view! {
                    <input
                        value=value
                        type=column.html_input_type
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
                            view! { <TableCell>{cell.into_view()}</TableCell> }
                        })
                        .collect_view()}
                </tr>
            }
        })
        .collect_view()
}

#[component]
pub fn TableCell(children: Children) -> impl IntoView {
    let cell_ellipsis_class = "text-ellipsis overflow-hidden";
    let cell_class = format!(
        "{} {} text-table-row-text-color font-medium text-sm text-left whitespace-nowrap max-w-40",
        CELL_PADDING_CLASS, cell_ellipsis_class,
    );
    view! { <td class=cell_class>{children()}</td> }
}

#[component]
pub fn EmptyTable(#[prop(into)] message: String) -> impl IntoView {
    view! {
        <div class="w-full flex text-base text-slate-400 items-center justify-center p-8">
            <NoIcon/>
            <span class="pl-4 text-sm">{message}</span>
        </div>
    }
}

#[component]
pub fn TableSection<E, F>(
    #[prop(into)] section_heading: String,
    children: Children,
    #[prop(optional, into)] additional_info: View,
    metadata: Signal<Option<TableMetadata>>,
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
                    {move || {
                        metadata
                            .get()
                            .map(|m| {
                                view! {
                                    <div class="metadata pl-4 ".to_string()
                                        + BASE_META_CLASS>{format_metadata(&m, format_number)}</div>
                                }
                                    .into_view()
                            })
                    }}

                </div>
                <div class="grow md:grow-0 h-16 flex justify-end items-center pr-4">
                    {controls()}
                </div>
            </span>
            <div class="additional-info hidden empty:hidden md:flex w-full pl-8 pr-4 h-10 grow flex justify-between items-start">
                {additional_info}
            </div>
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
                class="font-bold uppercase text-sm flex justify-center align-center ".to_string()
                    + LINK_HOVER_STATE
            >
                {children()}
                <span class="mx-1">{text}</span>
                <ChevronRight/>
            </a>
        </div>
    }
    .into_view()
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

#[derive(Clone)]
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
