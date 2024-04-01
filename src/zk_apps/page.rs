use super::functions::*;
use crate::common::{components::*, functions::*, table::*};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn ZkAppTransactionsPage() -> impl IntoView {
    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);
    let data = stub_zk_app_trx_data(1000);
    view! {
        <Title text="Transactions | ZK Apps"/>
        <PageContainer>
            <TableSection section_heading="ZK App Transactions" controls=|| ().into_view()>

                {move || {
                    let data = data.clone();
                    let pag = build_pagination(
                        data.len(),
                        records_per_page,
                        current_page.get(),
                        set_current_page,
                    );
                    let subset = get_subset(
                        &data.into_iter().map(Some).collect::<Vec<_>>(),
                        records_per_page,
                        current_page.get() - 1,
                    );
                    view! { <Table data=subset pagination=pag/> }
                }}

            </TableSection>
        </PageContainer>
    }
}

#[component]
pub fn ZkAppsPage() -> impl IntoView {
    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);
    let data = stub_zk_apps_data(9000);
    view! {
        <Title text="ZK Apps | Search For ZK Apps"/>
        <PageContainer>
            <TableSection section_heading="ZK Apps" controls=|| ().into_view()>

                {move || {
                    let data = data.clone();
                    let pag = build_pagination(
                        data.len(),
                        records_per_page,
                        current_page.get(),
                        set_current_page,
                    );
                    let subset = get_subset(
                        &data.into_iter().map(Some).collect::<Vec<_>>(),
                        records_per_page,
                        current_page.get() - 1,
                    );
                    view! { <Table data=subset pagination=pag/> }
                }}

            </TableSection>
        </PageContainer>
    }
}
