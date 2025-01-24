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
          <time class="text-xl md:w-28 font-tiny text-nord10">{date}</time>
        </div>
        <div class="ml-14 text-xl font-tiny text-nord4">
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

  view! { <ul class="pl-4 mb-2 list-outside text-md list-[square] text-nord4">{children}</ul> }
}

pub(crate) enum MediaProperties {
  Contain,
  Cover,
}

pub(crate) struct Card {
  pub media_path: String,
  pub media_props: MediaProperties,
  pub rotation: String,
  pub description: String,
  pub link: String,
}

#[component]
pub(crate) fn CardShowcase(cards: Vec<Card>) -> impl IntoView {
  view! {
    <div class="flex overflow-visible relative flex-col mx-auto lg:flex-row">
      <div class="flex relative flex-col mx-auto lg:flex-row">
        {cards
          .into_iter()
          .map(|card| {
            let media_props = match card.media_props {
              MediaProperties::Contain => "object-contain p-2 mx-auto",
              MediaProperties::Cover => "object-cover w-full",
            };
            view! {
              <a href=card.link class="flex relative z-0 p-4 hover:z-50 group">
                <div class=format!(
                  "relative rounded-xl {} hover:rotate-0 duration-500 hover:translate-y-10 h-full \
                  lg:w-[250px] 2xl:w-[300px] 3xl:w-[400px] object-cover hover:scale-105 \
                  lg:hover:scale-200 xl:hover:scale-200 2xl:hover:scale-150 transform origin-bottom",
                  card.rotation,
                )>
                  <div
                    class="absolute inset-0 bg-center bg-cover rounded-xl"
                    style="background-image: url('/images/image-1.png');"
                  ></div>
                  <img
                    class=format!(
                      "relative h-full opacity-90 rounded-xl transition-all group-hover:opacity-100 group-hover:filter-none {}",
                      media_props,
                    )
                    src=card.media_path
                  />
                  <div class="absolute top-0 right-0 pr-2 pl-2 w-full rounded-t-xl opacity-0 transition-all duration-300 group-hover:right-0 group-hover:opacity-100 bg-nord0/50 backdrop-blur-sm">
                    <p class="text-md md:text-md lg:text-sm font-silkscreen text-nord4">{card.description}</p>
                  </div>
                  <div class="inline-flex absolute right-0 bottom-0 items-center px-2 m-4 space-x-2 text-xl font-bold rounded-sm opacity-0 transition-all duration-300 transform lg:text-xs xl:text-xs 2xl:text-sm group-hover:opacity-100 outline-2 font-tiny bg-nord15 group-hover:outline group-hover:outline-nord8 group-hover:bg-nord0 group-hover:text-nord8 hover:outline hover:outline-nord8 hover:bg-nord0 hover:text-nord8 active:bg-nord2 active:outline-nord13 active:text-nord13">
                    <span>Read more on</span>
                    <lucide_leptos::Github size=16 />
                  </div>
                </div>
              </a>
            }
          })
          .collect::<Vec<_>>()}
      </div>
    </div>
  }
}
