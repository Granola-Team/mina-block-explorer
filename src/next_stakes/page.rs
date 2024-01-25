use super::functions::*;
use crate::common::components::*;
use crate::common::search::*;
use crate::common::table::*;
use crate::stakes::components::StakesNavButton;
use leptos::*;

#[component]
pub fn NextStakesPage() -> impl IntoView {
    let resource = create_resource(|| (), |_| async move { load_data(10).await });

    view! {
        <SearchBar placeholder="Exact search for public key".to_string()/>
        <PageContainer>
            {move || match resource.get() {
                Some(Ok(data)) => view! {
                        <TableSection section_heading="Next Epoch Staking Ledger".to_string() controls=move || view! {
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
