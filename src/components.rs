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
      <div class="relative my-12 space-y-8 before:absolute before:inset-0 before:ml-5 before:-translate-x-px before:h-full before:w-0.5 before:bg-nord3 before:to-transparent md:before:ml-[8.2rem] md:before:translate-x-0">
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
  url: Option<String>,
  bullet_points: Vec<String>,
) -> impl IntoView {
  view! {
    <div class="relative">
      <div class="items-center mb-3 md:flex md:space-x-4">
        <div class="flex items-center space-x-8 md:space-x-2 md:space-x-reverse">
          <div class="flex justify-center items-center ml-4 w-2 h-2 md:order-1 bg-nord3 md:ml-[0.5rem]"></div>
          <time class="text-2xl md:w-28 font-m5 text-nord10">{date}</time>
        </div>
        <div class="ml-14 text-xl font-retro text-nord4">
          {move || {
            let sub_title = sub_title.clone();
            let url = url.clone();
            if let Some(url) = url {
              return view! {
                <a href=url class="group">
                  <span class="mr-4 underlined text-nord15 hover:text-nord8">{sub_title}</span>
                </a>
              }
                .into_any()
            } else {
              return view! { <span class="mr-4 text-nord15">{sub_title}</span> }.into_any()
            }
          }} {title}
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
    .map(|child| view! { <li class="mb-2 bullet-point">{child}</li> })
    .collect::<Vec<_>>();

  view! { <ul class="pl-4 mb-2 text-sm list-outside list-[square] text-nord4">{children}</ul> }
}

pub(crate) enum MediaProperties {
  Contain,
  Cover,
}

#[component]
pub(crate) fn Showcase(media_path: String, media_props: MediaProperties, description: String, link: String) -> impl IntoView {
  let media_props = match media_props {
    MediaProperties::Contain => "object-contain p-4 group-hover:scale-105 mx-auto",
    MediaProperties::Cover => "object-cover w-full",
  };
  view! {
    <a href=link.clone() rel="external" target="tab">
      <div class="overflow-hidden relative justify-self-start self-start place-content-start mb-8 w-full h-96 showcase group">
        <div class="absolute inset-0 bg-center bg-cover" style="background-image: url('/images/image-1.png');"></div>
        <img
          class=format!(
            "relative h-full opacity-90 transition-all 2xl:rounded-xl group-hover:opacity-100 group-hover:filter-none {}",
            media_props,
          )
          src=media_path.clone()
        />
        <div class="absolute top-0 right-0 p-2 m-8 max-w-screen-md text-right rounded-xl transition-transform group-hover:right-0 group-hover:scale-105 bg-nord0/50 backdrop-blur-sm duration-250">
          <p class="text-3xl font-m5 text-nord4">{description}</p>
        </div>

        <div class="inline-flex absolute right-0 bottom-0 items-center m-8 space-x-5 default-button">
          <span>Read more on</span>
          <lucide_leptos::Github />
        </div>
      </div>
    </a>
  }
}
