use leptos::*;

pub trait TableData {
    fn get_columns(&self) -> Vec<String>;
    fn get_rows(&self) -> Vec<Vec<String>>;
}

#[derive(Clone)]
pub struct Pagination {
    pub current_page: i32,
    pub records_per_page: i32,
    pub total_records: i32,
    pub next_page: fn(),
    pub prev_page: fn()
}

impl Pagination {
    fn start_index(&self) -> i32 {
        self.current_page * self.records_per_page - self.records_per_page + 1 
    }

    fn end_index(&self) -> i32 {
        self.current_page * self.records_per_page
    }

    fn total_pages(&self) -> i32 {
        self.total_records / self.records_per_page + (self.total_records % self.records_per_page).signum()
    }

}

#[test]
fn test_indexes_first_page() {
    
    let pd = Pagination {
        current_page: 1,
        records_per_page: 15,
        total_records: 90,
        next_page: todo!(),
        prev_page: todo!(),
    };
    assert_eq!(pd.start_index(), 1);
    assert_eq!(pd.end_index(), 15)
}

#[test]
fn test_indexes_second_page() {
    let pd = Pagination {
        current_page: 2,
        records_per_page: 15,
        total_records: 90,
        next_page: todo!(),
        prev_page: todo!(),
    };
    assert_eq!(pd.start_index(), 16);
    assert_eq!(pd.end_index(), 30)
}

#[test]
fn test_total_pages() {
    let pd = Pagination {
        current_page: 2,
        records_per_page: 15,
        total_records: 90,
        next_page: todo!(),
        prev_page: todo!(),
    };
    assert_eq!(pd.total_pages(), 6);
    let pd = Pagination {
        current_page: 2,
        records_per_page: 15,
        total_records: 91,
        next_page: todo!(),
        prev_page: todo!(),
    };
    assert_eq!(pd.total_pages(), 7);
}

#[component]
pub fn Table<T>(
    data: T, 
    #[prop(optional)]
    pagination: Option<Pagination>) -> impl IntoView
where 
    T: TableData
{
    let columns = data.get_columns();
    let rows = data.get_rows();

    view! {
        <div class="w-full overflow-x-auto">
            <table class="md:rounded-b-lg table-fixed w-[300%] md:w-[150%] lg:w-full ">
            <tr class="h-12 bg-table-header-fill">
                {columns.into_iter()
                    .map(|s| view! { <th class="first:pl-8 text-table-header-text-color font-semibold uppercase text-xs text-left">{s}</th>})
                    .collect::<Vec<_>>()}
            </tr>
            {rows.into_iter()
                .map(|row| view! {
                    <tr class="h-12 bg-table-row-fill">
                        {row.into_iter().map(|cell| view! { <td class="first:pl-8 text-table-row-text-color font-medium text-sm text-left overflow-hidden whitespace-nowrap text-ellipsis">{cell}</td>}).collect::<Vec<_>>()}
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
                        <div class="flex">
                            <span>
                                {format!("Showing {} to {} of {} records", pg.start_index(), pg.end_index(), pg.total_records)}
                            </span>
                            <span>
                                <button on:click=move |_| (pg.prev_page)()>"<< Previous Page"</button>
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
