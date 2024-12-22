use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;

fn main() {
  console_error_panic_hook::set_once();
  leptos::mount::mount_to_body(|| view! { <p class="text-xl">"Hello, world!"</p> })
}
