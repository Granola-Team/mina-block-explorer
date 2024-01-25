use super::functions::*;
use crate::common::components::*;
use crate::common::search::*;
use crate::common::table::*;
use crate::stakes::components::StakesNavButton;
use leptos::*;
use leptos_router::*;

#[component]
pub fn NextStakesPage() -> impl IntoView {
    let query_params_map = use_query_map();
    let resource = create_resource(
        move || query_params_map.get(),
        |params_map| async move {
            let public_key = params_map.get("query");
            load_data(10, public_key.cloned()).await
        },
    );

    view! {
        <SearchBar placeholder="Exact search for public key".to_string()/>
        <PageContainer>
            {move || match resource.get() {
                Some(Ok(data)) => view! {
                        <TableSection section_heading="Next Staking Ledger".to_string() controls=move || view! {
                            <StakesNavButton href="/stakes".to_string() text="Current Stakes".to_string() />
                        }>
                            <Table data=data.nextstakes/>
                        </TableSection>
                    }
                ,
                None => view! {
                    <TableSection section_heading=String::new() controls=move || ()>
                        <Table data=LoadingPlaceholder{}/>
                    </TableSection>
                },
                _ => view! { <NullView /> }
            }}
        </PageContainer>
    }
}
