use leptos::*;
use leptos_router::*;

use crate::nav::Nav;
use crate::summary::Summary;

#[component]
pub fn Root() -> impl IntoView {
  view! {
    <Nav />
    <main>
      <Router>
        <Routes>
          <Route path="/" view=Summary />
          <Route path="/summary" view=Summary />
        </Routes>
      </Router>
    </main>
  }
}