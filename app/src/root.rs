use leptos::*;
use leptos_router::*;

use crate::account_page::AccountSummary;
use crate::nav::Nav;
use crate::summary_page::SummaryPage;

#[component]
pub fn Root() -> impl IntoView {
    view! {
      <Nav />
      <main class="m-1.5">
        <Router>
          <Routes>
            <Route path="/" view=SummaryPage />
            <Route path="/summary" view=SummaryPage />
            <Route path="/accounts/:id" view=AccountSummary />
          </Routes>
        </Router>
      </main>
    }
}
