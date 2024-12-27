mod pages;

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use crate::pages::*;

fn main() {
  console_error_panic_hook::set_once();
  mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
  view! {
    <Router>
      <nav>
        <div class="navbar-item">
          <A href="/">Home</A>
        </div>
        <div class="navbar-item">
          <A href="/blog">Blog</A>
        </div>
      </nav>
      <main>
        <Routes fallback=|| "Not found.">
          <Route path=path!("/") view=Home />
          <Route path=path!("/blog") view=Blog />
        </Routes>
      </main>
    </Router>
  }
}
