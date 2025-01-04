use leptos::{component, view, IntoView};
use leptos::prelude::*;

#[component]
pub(crate) fn Tag(tag: String) -> impl IntoView {
  view! { <span class="mr-2 tag">#{tag.to_lowercase()}</span> }
}

#[component]
pub(crate) fn Timeline(children: Children) -> impl IntoView {
  view! {
    <div class="relative py-24 my-12 space-y-8 before:absolute before:inset-0 before:ml-5 before:-translate-x-px before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-nord7 before:to-transparent md:before:ml-[8.75rem] md:before:translate-x-0">
      {children()}
    </div>
  }
}

#[component]
pub(crate) fn TimelineEntry(
  date: String,
  sub_title: String,
  title: String,
  bullet_points: Vec<String>,
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
      <div class="p-4 ml-14 rounded md:ml-44 bg-nord1 text-nord6">
        <BulletPoints>{bullet_points}</BulletPoints>
      </div>
    </div>
  }
}

#[component]
pub fn BulletPoints(children: ChildrenFragment) -> impl IntoView {
  let children = children()
    .nodes
    .into_iter()
    .map(|child| view! { <li>{child}</li> })
    .collect::<Vec<_>>();

  view! { <ul class="pl-4 list-outside list-[square] text-nord9">{children}</ul> }
}