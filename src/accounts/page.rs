use crate::{
    accounts::functions::stub_account_summaries,
    common::{components::*, table::*},
};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn AccountsPage() -> impl IntoView {
    view! {
        <Title text="Accounts | Search For Mina Account"/>
        <PageContainer>
            <AccountsPageContents/>
        </PageContainer>
    }
}

#[component]
fn AccountsPageContents() -> impl IntoView {
    let data = stub_account_summaries(100);
    view! {
        <TableSection section_heading="Accounts" controls=|| ().into_view()>
            <DeprecatedTable data=data/>
        </TableSection>
    }
}
