use leptos::{component, view, IntoView};
use leptos::prelude::*;

#[component]
pub(crate) fn Tag(tag: String) -> impl IntoView {
  view! { <span class="mr-2 tag">#{tag.to_lowercase()}</span> }
}

#[component]
pub(crate) fn TimelineEntry(
  date: String,
  sub_title: String,
  title: String,
  description: String,
  icon: impl IntoView,
) -> impl IntoView {
  view! {
    <div class="relative">
      <div class="items-center mb-3 md:flex md:space-x-4">
        <div class="flex items-center space-x-4 md:space-x-2 md:space-x-reverse">
          <div class="flex justify-center items-center w-10 h-10 bg-white rounded-full shadow md:order-1">{icon}</div>
          <time class="text-2xl md:w-28 font-m5 text-nord10">{date}</time>
        </div>
        <div class="ml-14 text-xl font-retro text-nord4">
          <span class="mr-4 text-nord4 text-nord8">{sub_title}</span>
          {title}
        </div>
      </div>
      <div class="p-4 ml-14 rounded md:ml-44 bg-nord1 text-nord6">{description}</div>
    </div>
  }
}