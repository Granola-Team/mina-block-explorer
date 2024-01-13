use leptos::*;
use leptos_router::*;

use crate::accounts::account_dialog::AccountDialogView;
use crate::accounts::page::{AccountSummaryPage, AccountsPage};
use crate::blocks::page::{BlockSpotlight, LatestBlocksPage};
use crate::common::components::NullView;
use crate::common::search::SearchBar;
use crate::footer::Footer;
use crate::header::navigation::Header;
use crate::snarks::page::SnarksPage;
use crate::stakes_page::StakesPage;
use crate::summary::page::SummaryPage;
use crate::transactions::page::{TransactionSpotlightPage, TransactionsPage};

#[component]
fn PageWrapper(children: Children) -> impl IntoView {
    view! {
      <main class="grid grid-cols-1 md:grid-cols-[10%_80%_10%] bg-secondary-background rounded-t-3xl py-6 sm:px-0 grow">
        {children()}
      </main>
    }
}

#[component]
pub fn Root() -> impl IntoView {
    view! {
        <Router>
          <Header />
            <Routes>
              <Route path="/" view={move || view!{<PageWrapper><SummaryPage /></PageWrapper>}} />
              <Route path="/summary" view={move || view!{<PageWrapper><SummaryPage/></PageWrapper>}}>
                <Route path="accounts/:id" view={move || view!{<PageWrapper><AccountDialogView/></PageWrapper>}} />
                <Route path="" view=NullView/>
              </Route>
              <Route path="/accounts" view={move || view!{
                <SearchBar />
                <PageWrapper><AccountsPage/></PageWrapper>
              }} />
              <Route path="/accounts/:id" view={move || view!{<PageWrapper><AccountSummaryPage/></PageWrapper> }} />
              <Route path="/blocks" view={move || view!{<PageWrapper><LatestBlocksPage/></PageWrapper>}} />
                <Route path="accounts/:id" view={move || view!{<PageWrapper><AccountDialogView/></PageWrapper>}}>
                <Route path="" view=NullView/>
              </Route>
              <Route path="/blocks/:id" view={move || view!{<PageWrapper><BlockSpotlight/></PageWrapper>}} />
              <Route path="/transactions" view={move || view!{<PageWrapper><TransactionsPage/></PageWrapper>}} />
              <Route path="/transactions/:id" view={move || view!{<PageWrapper><TransactionSpotlightPage/></PageWrapper>}} />
              <Route path="/snarks" view={move || view!{<PageWrapper><SnarksPage/></PageWrapper> }} />
              <Route path="/stakes" view={move || view!{<PageWrapper><StakesPage/></PageWrapper> }} />
            </Routes>
          <Footer />
        </Router>

    }
}
