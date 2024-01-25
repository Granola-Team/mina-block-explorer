use leptos::*;
use leptos_router::*;

use crate::accounts::account_dialog::AccountDialogView;
use crate::accounts::page::{AccountSpotlightPage, AccountsPage};
use crate::blocks::page::{BlockSpotlight, LatestBlocksPage};
use crate::common::components::NullView;
use crate::footer::Footer;
use crate::header::navigation::Header;
use crate::next_stakes::page::NextStakesPage;
use crate::snarks::page::SnarksPage;
use crate::stakes::page::StakesPage;
use crate::summary::page::SummaryPage;
use crate::transactions::page::{TransactionSpotlightPage, TransactionsPage};

#[component]
pub fn Root() -> impl IntoView {
    view! {
        <Router>
          <Header />
          <main>
            <Routes>
              <Route path="/" view=SummaryPage />
              <Route path="/summary" view=SummaryPage>
                <Route path="accounts/:id" view=AccountDialogView/>
                <Route path="" view=NullView/>
              </Route>
              <Route path="/accounts" view=AccountsPage />
              <Route path="/accounts/:id" view=AccountSpotlightPage />
              <Route path="/blocks" view=LatestBlocksPage>
                <Route path="accounts/:id" view=AccountDialogView/>
                <Route path="" view=NullView/>
              </Route>
              <Route path="/blocks/:id" view=BlockSpotlight/>
              <Route path="/transactions" view=TransactionsPage/>
              <Route path="/transactions/:id" view=TransactionSpotlightPage/>
              <Route path="/snarks" view=SnarksPage />
              <Route path="/stakes" view=StakesPage />
              <Route path="/next-stakes" view=NextStakesPage />
            </Routes>
          </main>
          <Footer />
        </Router>

    }
}
