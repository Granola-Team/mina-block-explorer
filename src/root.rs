use leptos::*;
use leptos_router::*;

use crate::accounts::account_dialog::AccountDialogView;
use crate::accounts::page::AccountSummaryPage;
use crate::blocks::page::{BlockSpotlight, LatestBlocksPage};
use crate::footer::Footer;
use crate::header::navigation::Header;
use crate::snarks::page::SnarksPage;
use crate::stakes_page::StakesPage;
use crate::summary::page::SummaryPage;
use crate::transactions::page::TransactionsPage;

#[component]
pub fn Root() -> impl IntoView {
    view! {
        <Router>
          <Header />
          <main class="grid grid-cols-1 md:grid-cols-[10%_80%_10%] bg-secondary-background rounded-t-3xl py-6 px-2 sm:px-0 grow">
            <Routes>
              <Route path="/" view=SummaryPage />
              <Route path="/summary" view=SummaryPage>
                <Route path="accounts/:id" view=AccountDialogView/>
                <Route path="" view=|| view! {}.into_view()/>
              </Route>
              <Route path="/accounts/:id" view=AccountSummaryPage />
              <Route path="/blocks" view=LatestBlocksPage>
                <Route path="accounts/:id" view=AccountDialogView/>
                <Route path="" view=|| view! {}.into_view()/>
              </Route>
              <Route path="/blocks/:id" view=BlockSpotlight/>
              <Route path="/transactions" view=TransactionsPage />
              <Route path="/snarks" view=SnarksPage />
              <Route path="/stakes" view=StakesPage />
            </Routes>
          </main>
          <Footer />
        </Router>

    }
}
