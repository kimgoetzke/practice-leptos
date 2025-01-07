use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub(crate) fn Tag(tag: String) -> impl IntoView {
  view! { <span class="mr-2 tag">#{tag.to_lowercase()}</span> }
}

#[component]
pub(crate) fn Timeline(children: Children) -> impl IntoView {
  view! {
    <div class="justify-self-start self-start place-content-start">
      <div class="relative py-6 my-12 space-y-8 before:absolute before:inset-0 before:ml-5 before:-translate-x-px before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-nord7 before:to-transparent md:before:ml-[8.75rem] md:before:translate-x-0">
        {children()}
      </div>
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
      {move || {
        let bullet_points = bullet_points.clone();
        if !bullet_points.is_empty() {
          return view! {
            <div class="p-4 ml-14 rounded md:ml-44 bg-nord1 text-nord6">
              <BulletPoints>{bullet_points}</BulletPoints>
            </div>
          }
            .into_any()
        } else {
          return view! { <div></div> }.into_any()
        }
      }}
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

  view! { <ul class="pl-4 text-sm list-outside list-[square] text-nord4">{children}</ul> }
}

#[component]
pub(crate) fn Showcase(image_path: String, description: String, link: String) -> impl IntoView {
  view! {
    <div class="relative justify-self-start self-start place-content-start mb-8 w-full h-96 showcase">
      <img class="object-cover w-full h-full" src=image_path.clone() />
      <div class="absolute top-0 right-0 p-2 m-8 max-w-screen-md text-right rounded-xl bg-nord0/25 backdrop-blur-sm">
        <p class="text-3xl text-nord4 font-m5">{description}</p>
      </div>
      <a href=link.clone() rel="external" target="tab">
        <div class="inline-flex absolute right-0 bottom-0 items-center m-8 space-x-5 default-button">
          <span>Read more on</span>
          <lucide_leptos::Github />
        </div>
      </a>
    </div>
  }
}
