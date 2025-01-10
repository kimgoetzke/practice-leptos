mod components;
mod pages;

use crate::pages::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

fn main() {
  _ = console_log::init_with_level(log::Level::Debug);
  console_error_panic_hook::set_once();
  mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
  provide_meta_context();

  view! {
    <Title text="Kim Goetzke" />
    <Meta charset="UTF-8" />
    <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <Router>
      <nav>
        <div class="inline-flex flex-row p-2 mx-12 space-x-6 rounded-full bg-nord0/25 backdrop-blur-sm">
          <A href="/">
            <div class="navbar-text-item">Home</div>
          </A>
          <A href="/blog">
            <div class="navbar-text-item">Blog</div>
          </A>
          <A href="https://github.com/kimgoetzke">
            <div class="navbar-item">
              <lucide_leptos::Github />
            </div>
          </A>
          <A href="https://uk.linkedin.com/in/kimgoetzke">
            <div class="navbar-item">
              <lucide_leptos::Linkedin />
            </div>
          </A>
          <A href="https://kimgoetzke.github.io/tags/">
            <div class="navbar-item">
              <lucide_leptos::NotebookPen />
            </div>
          </A>
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
    <div class="full-height-container">
      <h1>404</h1>
      <div class="py-6"></div>
      <p>Sorry, something went wrong.</p>
    </div>
  }
}
