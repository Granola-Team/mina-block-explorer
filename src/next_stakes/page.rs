use super::functions::*;
use crate::common::components::*;
use crate::common::search::*;
use crate::common::table::*;
use crate::common::functions::*;
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
            load_data(50, public_key.cloned()).await
        },
    );

    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        <SearchBar placeholder="Exact search for public key".to_string()/>
        <PageContainer>
            {move || match resource.get() {
                Some(Ok(data)) => {
                    let pag = build_pagination(data.nextstakes.len(), records_per_page, current_page.get(), set_current_page);
                    let subset = get_subset(&data.nextstakes, records_per_page, current_page.get()-1);
                    view! {
                        <TableSection section_heading="Next Staking Ledger".to_string() controls=move || view! {
                            <StakesNavButton href="/stakes".to_string() text="Current Stakes".to_string() />
                        }>
                            <Table data=subset pagination=pag/>
                        </TableSection>
                    }
                },
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
