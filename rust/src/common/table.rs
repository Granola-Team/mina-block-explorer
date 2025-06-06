use super::{components::*, functions::*, models::*};
use crate::{
    account_activity::models::Delegators,
    accounts::models::AccountsSort,
    analytics::{
        models::{SnarkerLeaderboardHighestFees, SnarkerLeaderboardTotalFees},
        staker_leaderboard::models::ExtendedTopStakersSortByInput,
    },
    common::constants::*,
    icons::*,
    stakes::models::StakesSort,
};
use heck::{ToKebabCase, ToTitleCase};
use leptos::{html::*, *};
use leptos_router::*;
use leptos_use::{DebounceOptions, use_debounce_fn_with_options};

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

#[derive(Clone, PartialEq)]
pub enum ColumnSearchType {
    None,
    Text,
    Select,
}

#[derive(Clone)]
pub struct TableColumn<T> {
    pub column: String,
    pub search_type: ColumnSearchType,
    pub search_options: Option<Vec<String>>,
    pub sort_direction: Option<T>,
    pub is_sortable: bool,
    pub width: Option<String>,
    pub html_input_type: String,
    pub alignment: Option<ColumnTextAlignment>,
    pub tooltip: Option<String>,
}

impl<T> Default for TableColumn<T> {
    fn default() -> Self {
        TableColumn {
            column: String::new(),
            search_type: ColumnSearchType::None,
            search_options: None,
            sort_direction: None,
            is_sortable: false,
            width: None,
            html_input_type: "text".to_string(),
            alignment: None,
            tooltip: None,
        }
    }
}

const INPUT_CLASS: &str = " block w-full mt-1 h-7 text-base text-sm font-normal font-mono rounded ";
const CELL_PADDING_CLASS: &str = " first:pl-8 pl-4 last:pr-4 ";

#[component]
pub fn Table(children: Children, id: String) -> impl IntoView {
    view! {
        <table
            id=id.to_string()
            data-test=id.to_string() + "-table"
            class="xl:relative font-mono md:rounded-b-lg w-full @xs:w-[400%] @md:w-[300%] @2xl:w-[200%] xl:w-full"
        >
            {children()}
        </table>
    }
}

pub trait SortDirection {
    fn is_desc(&self) -> bool;
    fn is_active(&self) -> bool;
}

pub trait CycleSort {
    fn cycle(&self) -> AnySort;
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
    TopStakersSortByInput(ExtendedTopStakersSortByInput),
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
            AnySort::TopStakersSortByInput(sort) => sort.is_desc(),
            AnySort::Stakes(sort) => sort.is_desc(),
        }
    }
    fn is_active(&self) -> bool {
        !matches!(
            self,
            AnySort::None(_)
                | AnySort::SnarkerLeaderboardHighestFee(SnarkerLeaderboardHighestFees::Nil)
                | AnySort::SnarkerLeaderboardTotalFees(SnarkerLeaderboardTotalFees::Nil)
                | AnySort::TopStakersSortByInput(ExtendedTopStakersSortByInput::SlotsNil)
                | AnySort::TopStakersSortByInput(ExtendedTopStakersSortByInput::CanonicalBlocksNil)
        )
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
            AnySort::TopStakersSortByInput(sort) => sort.to_string(),
            AnySort::Stakes(sort) => sort.to_string(),
        }
    }
}

impl CycleSort for AnySort {
    fn cycle(&self) -> AnySort {
        match self {
            AnySort::None(_) => AnySort::None(Nil),
            AnySort::Stakes(sort) => sort.cycle(),
            AnySort::Accounts(sort) => sort.cycle(),
            AnySort::SnarkerLeaderboardHighestFee(sort) => sort.cycle(),
            AnySort::SnarkerLeaderboardTotalFees(sort) => sort.cycle(),
            AnySort::TopStakersSortByInput(sort) => sort.cycle(),
            _ => AnySort::None(Nil),
        }
    }
}

#[component]
pub fn TableSectionTemplate<T, S>(
    table_columns: Vec<TableColumn<S>>, // deprecated
    data_sig: ReadSignal<Option<T>>,
    is_loading: Signal<bool>,
    #[prop(optional)] metadata: Option<Signal<Option<TableMetadata>>>,
    #[prop(into)] section_heading: MaybeSignal<String>,
    #[prop(optional, into)] additional_info: ViewFn,
    #[prop(optional, into)] controls: ViewFn,
    #[prop(optional, into)] footer: ViewFn,
    #[prop(default = false)] half_width: bool,
) -> impl IntoView
where
    T: TableData + Clone + 'static,
    S: CycleSort + SortDirection + ToString + Clone + 'static,
{
    let table_cols_length = table_columns.len();

    view! {
        <TableSection
            metadata=metadata.unwrap_or_default()
            section_heading=section_heading.clone()
            controls
            additional_info
        >
            <div
                class="@container overflow-auto xl:overflow-visible w-full"
                class=("md:w-1/2", move || half_width)
                class=("mx-auto", move || half_width)
            >
                <Table id=section_heading.get().as_str().to_lowercase().to_kebab_case()>
                    <ColGroup columns=table_columns.clone() />
                    <TableHeader columns=table_columns.clone() />

                    {move || {
                        if is_loading.get() {
                            view! {
                                <TableRows
                                    columns=table_columns.clone()
                                    data=vec![
                                        vec![LoadingPlaceholder; table_cols_length];
                                        (TABLE_ROW_LIMIT / 2) as usize
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
                    if !is_loading.get()
                        && data_sig.get().map(|v| v.get_rows().len()).unwrap_or_default() == 0
                    {
                        view! { <EmptyTable message="No data for this view" /> }
                    } else {
                        ().into_view()
                    }
                }}
                {move || footer.run()}
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
    T: CycleSort + SortDirection + ToString + Clone + 'static,
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
    T: CycleSort + SortDirection + ToString + Clone + 'static,
{
    let id_copy = id.clone();
    let (value, set_value) = create_query_signal::<String>(id);
    let (_, set_sort_dir) = create_query_signal::<String>("sort-dir");
    let input_element: NodeRef<html::Input> = create_node_ref();
    let select_element: NodeRef<html::Select> = create_node_ref();
    let mut th_class = " whitespace-nowrap h-12 bg-table-header-fill xl:sticky xl:top-16 z-[1] text-table-header-text-color font-semibold uppercase text-xs text-left py-4 box-border ".to_string();
    let mut input_class = " pointer-events-auto ".to_string();
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

    let search_type = column.search_type.clone();

    let update_value = use_debounce_fn_with_options(
        move || {
            let v = match search_type {
                ColumnSearchType::Text => input_element
                    .get()
                    .expect("<input/> should be mounted")
                    .value(),
                ColumnSearchType::Select => select_element
                    .get()
                    .expect("<select/> should be mounted")
                    .value(),
                ColumnSearchType::None => panic!("Unexpected scenario"),
            };
            if v.is_empty() {
                set_value.set(None);
            } else {
                set_value.set(Some(v));
            }
        },
        DEFAULT_USER_INPUT_DEBOUNCE_INTERNVAL,
        DebounceOptions::default(),
    );

    let col_clone = column.clone();
    let col_clone_2 = col_clone.clone();
    view! {
        <th
            data-test=format!("column-{}", col_clone.column)
            class=th_class + CELL_PADDING_CLASS
            class=(
                "cursor-pointer",
                move || col_clone_2.sort_direction.is_some() || col_clone_2.is_sortable,
            )
            on:click=move |_| {
                if let Some(dir) = &col_clone.sort_direction {
                    set_sort_dir.set(Some(dir.cycle().to_string()))
                }
            }
        >
            <span class="flex justify-start items-center flex-wrap">
                {column
                    .tooltip
                    .map(|title| {
                        view! {
                            <span
                                class="flex justify-start items-center cursor-pointer"
                                title=title.to_string()
                            >
                                {column.column.to_string()}
                                <span class="w-2" />
                                {convert_to_tooltip(title).into_view()}
                            </span>
                        }
                            .into_view()
                    })
                    .unwrap_or(view! { {column.column.to_string()} }.into_view())}
                {match &column.sort_direction {
                    Some(direction) => {
                        if direction.is_active() {
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
                        } else {
                            view! {
                                <span class=ICON_CLASS>
                                    <UpDownArrow width=12 />
                                </span>
                            }
                                .into_view()
                        }
                    }
                    None => ().into_view(),
                }}
                {match column.search_type {
                    ColumnSearchType::Text => {
                        view! {
                            <input
                                data-test=format!("input-{}", column.column)
                                value=value
                                type=column.html_input_type
                                on:input=move |_| {
                                    update_value();
                                }
                                on:click=move |e| {
                                    e.stop_propagation();
                                }
                                node_ref=input_element
                                class=INPUT_CLASS.to_string() + &input_class + " p-2 pl-1"
                                id=id_copy
                            />
                        }
                            .into_view()
                    }
                    ColumnSearchType::Select => {
                        view! {
                            <select
                                data-test=format!("select-{}", column.column)
                                value=value
                                on:change=move |_| {
                                    update_value();
                                }
                                node_ref=select_element
                                class=INPUT_CLASS.to_string() + &input_class
                                    + " p-0 text-xs bg-white "
                                id=id_copy
                            >
                                {column
                                    .search_options
                                    .expect("Expected to have search options")
                                    .into_iter()
                                    .map(|text| {
                                        view! {
                                            <option
                                                value=text.to_string()
                                                selected=text == value.get_untracked().unwrap_or_default()
                                            >
                                                {ToTitleCase::to_title_case(text.as_str()).to_string()}
                                            </option>
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                    .into_view()}
                            </select>
                        }
                            .into_view()
                    }
                    _ => view! { <div class=INPUT_CLASS></div> }.into_view(),
                }}
            </span>
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
                <tr class="h-12 bg-table-row-fill border-b border-slate-200">
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
pub fn TableSection(
    #[prop(into)] section_heading: MaybeSignal<String>,
    children: Children,
    #[prop(optional, into)] additional_info: ViewFn,
    metadata: Signal<Option<TableMetadata>>,
    #[prop(optional, into)] controls: ViewFn,
) -> impl IntoView {
    let section_heading_clone = section_heading.clone();
    view! {
        <AppSection>
            <span class="w-full flex justify-between flex-wrap">
                <div class="flex justify-start items-baseline flex-wrap">
                    {move || {
                        view! { <AppHeading heading=section_heading_clone.get().to_string() /> }
                    }}
                    {move || {
                        metadata
                            .get()
                            .map(|meta| {
                                view! {
                                    <Metadata
                                        meta
                                        section_heading=section_heading
                                            .get()
                                            .as_str()
                                            .to_lowercase()
                                            .to_kebab_case()
                                    />
                                }
                            })
                    }}

                </div>
                <div class="grow md:grow-0 min-h-16 flex justify-end items-center flex-wrap pr-4 pb-2">
                    {move || controls.run()}
                </div>
            </span>
            <div class="additional-info hidden empty:hidden md:flex w-full pl-8 pr-4 h-10 grow flex justify-between items-start">
                {move || additional_info.run()}
            </div>
            {children()}
        </AppSection>
    }
}

#[component]
pub fn Metadata(meta: TableMetadata, section_heading: String) -> impl IntoView {
    let BASE_META_CLASS = "h-16 grow flex justify-start md:justify-center items-center text-slate-400 text-normal text-xs";
    let displayed = format_number(meta.displayed_records.to_string());
    let total = meta.total_records.map_or("?".to_string(), |records| {
        format_number(records.to_string())
    });

    view! {
        <div
            data-test=format!("metadata-{}", section_heading)
            class="metadata pl-4 ".to_string() + BASE_META_CLASS
        >
            <span>
                <span>{displayed.clone()}</span>
                {meta
                    .displayed_records_hint
                    .as_ref()
                    .map(|hint| convert_to_tooltip(hint.clone()))
                    .into_view()}
                <span>" of "</span>
                {if let Some(available_records) = meta.available_records {
                    let available = format_number(available_records.to_string());
                    view! {
                        <span>{available.clone()}</span>
                        {meta
                            .available_records_hint
                            .as_ref()
                            .map(|hint| convert_to_tooltip(hint.clone()))
                            .into_view()}
                        <span>" of "</span>
                    }
                        .into_view()
                } else {
                    ().into_view()
                }}
                <span>{total.clone()}</span>
                {meta
                    .total_records_hint
                    .as_ref()
                    .map(|hint| convert_to_tooltip(hint.clone()))
                    .into_view()}
            </span>
        </div>
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
