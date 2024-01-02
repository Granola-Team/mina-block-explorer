use leptos::*;
use leptos_router::*;


use crate::snarks_page::SnarksPage;
use crate::stakes_page::StakesPage;
use crate::styles::*;
use crate::summary_page::SummaryPage;
use crate::latest_block_page::LatestBlocksPage;
use crate::transactions::transactions_page::TransactionsPage;
use crate::header::navigation::Header;
use crate::accounts::account_dialog::AccountDialogView;
use crate::accounts::account_page::AccountSummaryPage;
use crate::footer::Footer;

#[component]
pub fn Root() -> impl IntoView {
    let breakout_container_styles = get_desktop_breakout_container_styles(None).to_styles_string();
    view! {    
        <Router>
          <Header />
          <main class={format!("bg-secondary-background rounded-t-3xl py-6 px-2 sm:px-0 grow {}",breakout_container_styles)}>
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
          </main>
          <Footer />
        </Router>
      
    }
}
