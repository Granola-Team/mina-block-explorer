use leptos::*;
use crate::common::components::*;
use super::functions::load_data;

#[component]
pub fn TransactionsSection(public_key: Option<String>) -> impl IntoView {
    let resource = create_resource(|| (), move |_| {
        let pk = public_key.clone();
        async move { 
            let limit = 10;
            load_data(limit, pk).await 
        }
    });

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! { 
                <TableSection section_heading="Transactions".to_owned()>
                    <Table data=data.transactions/>
                </TableSection>
             },
            _ => view! { <span /> }.into_view()
        }}
    }
}

#[component]
pub fn TransactionsPage() -> impl IntoView {
    view! {
        <TransactionsSection public_key=None/>
    }
}
