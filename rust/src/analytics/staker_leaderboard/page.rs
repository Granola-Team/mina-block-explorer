use crate::{analytics::staker_leaderboard::components::StakerLeaderboard, common::components::*};
use leptos::*;
use leptos_meta::Title;

#[component]
pub fn StakerLeaderboardPage() -> impl IntoView {
    view! {
        <Title text="Analytics | Staker Leaderboard" />
        <PageContainer>
            <AppSection>
                <StakerLeaderboard />
            </AppSection>
        </PageContainer>
    }
}
