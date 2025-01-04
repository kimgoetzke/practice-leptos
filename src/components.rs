use leptos::{component, view, IntoView};
use leptos::prelude::*;

#[component]
pub(crate) fn Tag(tag: String) -> impl IntoView {
  view! { <span class="mr-2 tag">#{tag.to_lowercase()}</span> }
}