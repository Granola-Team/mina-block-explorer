use leptos::*;
use std::collections::HashMap;


pub trait TableData {
    fn get_columns(&self) -> Vec<String>;
    fn get_rows(&self) -> Vec<Vec<String>>;
    fn get_linkable_cols(&self) -> HashMap<i32, String> {
        let linkcols: HashMap<i32, String> = HashMap::new();
        linkcols
    }
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
pub fn Table<T>(
    data: T, 
    #[prop(optional)]
    pagination: Option<Pagination>) -> impl IntoView
where 
    T: TableData
{
    let columns = data.get_columns();
    let rows = data.get_rows();
    let linkable_cols = data.get_linkable_cols();

    view! {
        <div class="w-full overflow-auto h-full">
            <table class="md:rounded-b-lg table-fixed w-[300%] md:w-[150%] lg:w-full ">
            <tr class="h-12 bg-table-header-fill">
                {columns.into_iter()
                    .map(|s| view! { <th class="first:pl-8 text-table-header-text-color font-semibold uppercase text-xs text-left">{s}</th>})
                    .collect::<Vec<_>>()}
            </tr>
            {rows.into_iter()
                .map(|row| view! {
                    <tr class="h-12 bg-table-row-fill">
                        {
                            row.iter().enumerate().map(|(index, cell)| {

                                let cell_class = "first:pl-8 text-table-row-text-color font-medium text-sm text-left overflow-hidden whitespace-nowrap text-ellipsis";
                                let cell_content = match linkable_cols.get(&(index as i32)) {
                                    Some(value) => {
                                        let link_href = value.replace(":token", cell);
                                        view! { <span><a href={link_href}>{cell}</a></span> } 
                                    }
                                    None => view! { <span>{cell}</span> }, 
                                };
                                view! {
                                    <td class={cell_class}>{cell_content}</td>
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
