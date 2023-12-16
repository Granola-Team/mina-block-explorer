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
       <table class="table-fixed w-full">
           <tr>
               {columns.into_iter()
                   .map(|s| view! { <th>{s}</th>})
                   .collect::<Vec<_>>()}
           </tr>
           {rows.into_iter()
            .map(|row| view! {
                <tr>
                    {row.into_iter().map(|cell| view! { <td class="overflow-hidden whitespace-nowrap text-ellipsis">{cell}</td>}).collect::<Vec<_>>()}
                </tr>
            })
            .collect::<Vec<_>>()}
       </table>
    }
}
