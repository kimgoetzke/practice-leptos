mod pages;
mod components;

use crate::pages::*;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

fn main() {
  console_error_panic_hook::set_once();
  mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
  view! {
    <Router>
      <nav>
        <div class="flex flex-row justify-center justify-items-center p-2 mx-12 rounded-full bg-nord0/25 backdrop-blur-sm">
          <div class="navbar-item">
            <A href="/">Home</A>
          </div>
          <div class="navbar-item">
            <A href="/blog">Blog</A>
          </div>
          <div class="navbar-item">
            <A href="https://github.com/kimgoetzke">
              <lucide_leptos::Github />
            </A>
          </div>
          <div class="navbar-item">
            <A href="https://kimgoetzke.github.io/tags/">
              <lucide_leptos::NotebookPen />
            </A>
          </div>
        </div>
      </nav>
      <main>
        <Routes fallback=|| NotFound>
          <Route path=path!("/") view=Home />
          <Route path=path!("/blog") view=Blog />
          <Route path=path!("/blog/:id") view=Post />
        </Routes>
      </main>
    </Router>
  }
}

#[component]
fn NotFound() -> impl IntoView {
  view! {
    <div class="centered-layout">
      <h1>404</h1>
      <div class="py-6"></div>
      <p>Sorry, something went wrong.</p>
    </div>
  }
}
