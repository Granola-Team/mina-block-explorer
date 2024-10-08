use super::{components::*, functions::*, models::*};
use crate::{
    account_activity::models::Delegators,
    accounts::models::AccountsSort,
    analytics::models::{
        SnarkerLeaderboardHighestFees, SnarkerLeaderboardTotalFees,
        StakerLeaderboardCanonicalBlocks,
    },
    common::constants::*,
    icons::*,
    stakes::models::StakesSort,
};
use heck::ToKebabCase;
use leptos::{html::*, *};
use leptos_router::*;
use leptos_use::{use_debounce_fn_with_options, DebounceOptions};

pub trait TableData {
    #[allow(dead_code)]
    fn get_columns(&self) -> Vec<String> {
        Vec::new()
    }
    fn get_rows(&self) -> Vec<Vec<HtmlElement<AnyElement>>>;
    #[allow(dead_code)]
    fn get_exact_search_columns(&self) -> Vec<String> {
        Vec::new()
    }
}

#[derive(Clone)]
pub enum ColumnTextAlignment {
    #[allow(dead_code)]
    Left,
    Right,
    #[allow(dead_code)]
    Center,
}

#[derive(Clone)]
pub struct TableColumn<T> {
    pub column: String,
    pub is_searchable: bool,
    pub sort_direction: Option<T>,
    pub is_sortable: bool,
    pub width: Option<String>,
    pub html_input_type: String,
    pub alignment: Option<ColumnTextAlignment>,
}

impl<T> Default for TableColumn<T> {
    fn default() -> Self {
        TableColumn {
            column: String::new(),
            is_searchable: false,
            sort_direction: None,
            is_sortable: false,
            width: None,
            html_input_type: "text".to_string(),
            alignment: None,
        }
    }
}

const INPUT_CLASS: &str =
    " block w-full mt-1 h-7 text-base text-sm font-normal font-mono p-2 rounded ";
const CELL_PADDING_CLASS: &str = " first:pl-8 pl-4 last:pr-4 ";

#[component]
pub fn Table(children: Children) -> impl IntoView {
    view! {
        <table class="xl:relative font-mono md:rounded-b-lg w-full @xs:w-[400%] @md:w-[300%] @2xl:w-[200%] xl:w-full">
            {children()}
        </table>
    }
}

pub trait SortDirection {
    fn is_desc(&self) -> bool;
}

pub trait NegateSort {
    fn negate(&self) -> AnySort;
}

#[derive(Clone)]
pub struct Nil;

#[derive(Clone)]
#[allow(dead_code)]
pub enum AnySort {
    None(Nil),
    SnarkerLeaderboardTotalFees(SnarkerLeaderboardTotalFees),
    SnarkerLeaderboardHighestFee(SnarkerLeaderboardHighestFees),
    Delegator(Delegators),
    Accounts(AccountsSort),
    StakerLeaderboardCanonicalBlocks(StakerLeaderboardCanonicalBlocks),
    Stakes(StakesSort),
}

impl SortDirection for AnySort {
    fn is_desc(&self) -> bool {
        match self {
            AnySort::None(_) => false,
            AnySort::SnarkerLeaderboardTotalFees(sort) => sort.is_desc(),
            AnySort::SnarkerLeaderboardHighestFee(sort) => sort.is_desc(),
            AnySort::Delegator(sort) => sort.is_desc(),
            AnySort::Accounts(sort) => sort.is_desc(),
            AnySort::StakerLeaderboardCanonicalBlocks(sort) => sort.is_desc(),
            AnySort::Stakes(sort) => sort.is_desc(),
        }
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for AnySort {
    fn to_string(&self) -> String {
        match self {
            AnySort::None(_) => String::new(),
            AnySort::SnarkerLeaderboardTotalFees(sort) => sort.to_string(),
            AnySort::SnarkerLeaderboardHighestFee(sort) => sort.to_string(),
            AnySort::Delegator(sort) => sort.to_string(),
            AnySort::Accounts(sort) => sort.to_string(),
            AnySort::StakerLeaderboardCanonicalBlocks(sort) => sort.to_string(),
            AnySort::Stakes(sort) => sort.to_string(),
        }
    }
}

impl NegateSort for AnySort {
    fn negate(&self) -> AnySort {
        match self {
            AnySort::None(_) => AnySort::None(Nil),
            AnySort::Stakes(sort) => sort.negate(),
            _ => AnySort::None(Nil),
        }
    }
}

#[component]
pub fn TableSectionTemplate<T, F, E, S>(
    table_columns: Vec<TableColumn<S>>,
    data_sig: ReadSignal<Option<T>>,
    is_loading: Signal<bool>,
    #[prop(optional)] metadata: Option<Signal<Option<TableMetadata>>>,
    #[prop(into)] section_heading: String,
    #[prop(optional, into)] additional_info: View,
    controls: F,
    #[prop(default = false)] half_width: bool,
) -> impl IntoView
where
    E: IntoView,
    F: Fn() -> E + 'static,
    T: TableData + Clone + 'static,
    S: NegateSort + SortDirection + ToString + Clone + 'static,
{
    let table_cols_length = table_columns.len();

    view! {
        <TableSection
            metadata=metadata.unwrap_or_default()
            section_heading
            controls
            additional_info
        >
            <div
                class="@container overflow-auto xl:overflow-visible w-full"
                class=("md:w-1/2", move || half_width)
                class=("mx-auto", move || half_width)
            >
                <Table>
                    <ColGroup columns=table_columns.clone() />
                    <TableHeader columns=table_columns.clone() />

                    {move || {
                        if is_loading.get() {
                            view! {
                                <TableRows
                                    columns=table_columns.clone()
                                    data=vec![
                                        vec![LoadingPlaceholder; table_cols_length];
                                        TABLE_ROW_LIMIT as usize
                                    ]
                                />
                            }
                        } else {
                            match data_sig.get() {
                                Some(data) => {
                                    view! { <TableRows columns=table_columns.clone() data=data /> }
                                }
                                None => ().into_view(),
                            }
                        }
                    }}

                </Table>
                {move || {
                    if let Some(data) = data_sig.get() {
                        if data.get_rows().is_empty() {
                            view! { <EmptyTable message="No data for this view" /> }
                        } else {
                            ().into_view()
                        }
                    } else {
                        view! { <EmptyTable message="No data for this view" /> }
                    }
                }}

            </div>
        </TableSection>
    }
}

#[component]
pub fn ColGroup<T>(columns: Vec<TableColumn<T>>) -> impl IntoView
where
    T: Clone + 'static,
{
    view! {
        {columns
            .iter()
            .map(|c| {
                c.width
                    .as_ref()
                    .map(|w| {
                        view! { <col width=w /> }
                    })
            })
            .collect_view()}
    }
}

#[component]
pub fn TableHeader<T>(columns: Vec<TableColumn<T>>) -> impl IntoView
where
    T: NegateSort + SortDirection + ToString + Clone + 'static,
{
    view! {
        <tr>
            {columns
                .iter()
                .map(|s| {
                    let id = "q-".to_string() + &s.column.as_str().to_kebab_case();
                    view! { <ColumnHeader id=id column=s.clone() /> }
                })
                .collect::<Vec<_>>()}
        </tr>
    }
}

const ICON_CLASS: &str = " inline-block cursor-pointer pl-1 items-center ";

#[component]
fn ColumnHeader<T>(id: String, column: TableColumn<T>) -> impl IntoView
where
    T: NegateSort + SortDirection + ToString + Clone + 'static,
{
    let id_copy = id.clone();
    let (value, set_value) = create_query_signal::<String>(id);
    let (_, set_sort_dir) = create_query_signal::<String>("sort-dir");
    let input_element: NodeRef<html::Input> = create_node_ref();
    let mut th_class = " whitespace-nowrap h-12 bg-table-header-fill xl:sticky xl:top-16 z-20 text-table-header-text-color font-semibold uppercase text-xs text-left py-4 box-border ".to_string();
    let mut input_class = "".to_string();
    match column.alignment {
        Some(ColumnTextAlignment::Left) => {
            th_class += " text-left ";
            input_class += " text-left ";
        }
        Some(ColumnTextAlignment::Right) => {
            th_class += " text-right ";
            input_class += " text-right ";
        }
        _ => (),
    }

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
        <th class=th_class
            + CELL_PADDING_CLASS>
            {column.column.clone()}
            {match (&column.sort_direction, column.is_sortable) {
                (Some(direction), _) => {
                    if direction.is_desc() {
                        view! {
                            <span class=ICON_CLASS>
                                <DownArrow width=12 />
                            </span>
                        }
                            .into_view()
                    } else {
                        view! {
                            <span class=ICON_CLASS>
                                <UpArrow width=12 />
                            </span>
                        }
                            .into_view()
                    }
                }
                (None, true) => view! {
                    <span class=ICON_CLASS>
                        <UpDownArrow width=12 />
                    </span>
                }.into_view(),
                (None, false) => ().into_view()
            }}
            {if column.is_searchable {
                view! {
                    <input
                        value=value
                        type=column.html_input_type
                        on:input=move |_| {
                            update_value();
                        }

                        node_ref=input_element
                        class=INPUT_CLASS.to_string() + &input_class
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
pub fn TableRows<T, S>(data: T, columns: Vec<TableColumn<S>>) -> impl IntoView
where
    T: TableData,
    S: Clone + 'static,
{
    data.get_rows()
        .into_iter()
        .map(|row| {
            view! {
                <tr class="h-12 bg-table-row-fill">
                    {row
                        .into_iter()
                        .enumerate()
                        .map(|(index, cell)| {
                            view! {
                                <TableCell column_opt=columns
                                    .get(index)
                                    .cloned()>{cell.into_view()}</TableCell>
                            }
                        })
                        .collect_view()}
                </tr>
            }
        })
        .collect_view()
}

#[component]
pub fn TableCell<T>(children: Children, column_opt: Option<TableColumn<T>>) -> impl IntoView
where
    T: Clone + 'static,
{
    let clss = " text-ellipsis overflow-hidden text-table-row-text-color font-medium text-sm whitespace-nowrap max-w-40 ".to_string();
    view! {
        <td class=CELL_PADDING_CLASS.to_string()
            + &clss>
            {if let Some(column) = column_opt {
                match column.alignment {
                    Some(ColumnTextAlignment::Left) => {
                        view! { <div class="flex items-center justify-start">{children()}</div> }
                            .into_view()
                    }
                    Some(ColumnTextAlignment::Right) => {
                        view! { <div class="flex items-center justify-end">{children()}</div> }
                            .into_view()
                    }
                    _ => children().into(),
                }
            } else {
                children().into()
            }}

        </td>
    }
}

#[component]
pub fn EmptyTable(#[prop(into)] message: String) -> impl IntoView {
    view! {
        <div class="w-full flex text-base text-slate-400 items-center justify-center p-8">
            <NoIcon />
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
                    <AppHeading heading=section_heading />
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
                <div class="grow md:grow-0 h-16 flex justify-end items-center flex-wrap pr-4">
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
                <ChevronRight />
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
