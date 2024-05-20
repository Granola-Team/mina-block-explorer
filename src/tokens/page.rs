use super::functions::*;
use crate::common::{components::*, constants::TABLE_ROW_LIMIT, table::*};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn TokensPage() -> impl IntoView {
    let data = stub_token_data(TABLE_ROW_LIMIT);
    view! {
        <Title text="Tokens | Search For Tokens"/>
        <PageContainer>
            <TableSection section_heading="Tokens" controls=|| ().into_view()>
                <DeprecatedTable data=data/>
            </TableSection>
        </PageContainer>
    }
}
