use leptos::*;

use crate::{
    table::{Table}, table_section::TableSection
};

use super::functions::load_data;

#[component]
pub fn TransactionsSection() -> impl IntoView {
    let resource = create_resource(|| (), |_| async move { 
        let limit = 10;
        load_data(limit, None).await 
    });

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! { 
                <TableSection section_heading="Transactions".to_owned()>
                    <Table data=data/>
                </TableSection>
             },
            _ => view! { <span /> }.into_view()
        }}
    }
}

#[component]
pub fn TransactionsPage() -> impl IntoView {
    view! {
        <TransactionsSection />
    }
}
