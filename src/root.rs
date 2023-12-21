use leptos::*;
use leptos_router::*;


use crate::snarks_page::SnarksPage;
use crate::stakes_page::StakesPage;
use crate::summary_page::SummaryPage;
use crate::latest_block_page::LatestBlocksPage;
use crate::transactions_page::TransactionsPage;
use crate::header::Header;
use crate::accounts::account_dialog::AccountDialogView;
use crate::accounts::account_page::AccountSummaryPage;

#[component]
pub fn Root() -> impl IntoView {
    view! {
      <Header />
      <main class="grid grid-cols-1 md:grid-cols-[10%_80%_10%] bg-secondary-background rounded-t-3xl pt-6 p-2 sm:p-0 sm:pt-6">
        <Router>
          <Routes>
            <Route path="/" view=SummaryPage />
            <Route path="/summary" view=SummaryPage>
              <Route path="accounts/:id" view=AccountDialogView/>
              <Route path="" view=|| view! { <span />}/>
            </Route>
            <Route path="/accounts/:id" view=AccountSummaryPage />
            <Route path="/blocks" view=LatestBlocksPage>
              <Route path="accounts/:id" view=AccountDialogView/>
              <Route path="" view=|| view! { <span />}/>
            </Route>
            <Route path="/transactions" view=TransactionsPage />
            <Route path="/snarks" view=SnarksPage />
            <Route path="/stakes" view=StakesPage />
          </Routes>
        </Router>
      </main>
    }
}
