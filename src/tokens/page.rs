use super::functions::*;
use crate::common::{components::*, table::*};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn TokensPage() -> impl IntoView {
    let data = stub_token_data(9000);
    view! {
        <Title text="Tokens | Search For Tokens"/>
        <PageContainer>
            <TableSection section_heading="Tokens" controls=|| ().into_view()>
                <DeprecatedTable data=data/>
            </TableSection>
        </PageContainer>
    }
}
