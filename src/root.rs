use leptos::*;
use leptos_router::*;

use crate::account_page::AccountSummary;
use crate::snarks_page::SnarksPage;
use crate::stakes_page::StakesPage;
use crate::summary_page::SummaryPage;
use crate::latest_block_page::LatestBlocksPage;
use crate::transactions_page::TransactionsPage;
use crate::header::Header;

#[component]
pub fn Root() -> impl IntoView {
    view! {
      <Header />
      <main class="mt-64">
        <Router>
          <Routes>
            <Route path="/" view=SummaryPage />
            <Route path="/summary" view=SummaryPage />
            <Route path="/accounts/:id" view=AccountSummary />
            <Route path="/blocks" view=LatestBlocksPage />
            <Route path="/transactions" view=TransactionsPage />
            <Route path="/snarks" view=SnarksPage />
            <Route path="/stakes" view=StakesPage />
          </Routes>
        </Router>
      </main>
    }
}
