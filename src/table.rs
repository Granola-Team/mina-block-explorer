use leptos::*;

pub trait TableData {
    fn get_columns(&self) -> Vec<String>;
    fn get_rows(&self) -> Vec<Vec<String>>;
}


#[component]
pub fn Table<T: TableData>(data: T) -> impl IntoView {
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
    }
}
