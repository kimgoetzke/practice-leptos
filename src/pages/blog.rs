use leptos::prelude::*;
use leptos::wasm_bindgen::JsValue;
use leptos::web_sys::js_sys::Date;
use leptos_router::components::*;
use crate::components::*;

#[component]
pub(crate) fn Blog() -> impl IntoView {
  view! {
    <div>
      <h2>Blog</h2>
      <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-4 3xl:grid-cols-5 4xl:grid-cols-6">
        {move || {
          let posts = crate::pages::blog_post::BLOG_POSTS.clone();
          posts
            .into_iter()
            .map(|post| {
              let date = Date::new(&JsValue::from_f64(post.created_at));
              let formatted_date = format!("{}", date.to_locale_date_string("en-GB", &JsValue::undefined()));
              view! {
                <div class="flex flex-col rounded-lg shadow-md transition hover:shadow-lg bg-nord1">
                  <div class="flex overflow-hidden justify-center items-center max-h-32 rounded-t-lg">
                    {move || {
                      post
                        .image
                        .as_ref()
                        .map(|img| view! { <img class="object-cover w-full h-full" src=img.clone() /> })
                    }}
                  </div>
                  <div class="p-4">
                    <h4 class="mb-2 text-xl font-semibold group text-nord4">
                      <A href=post.id.clone()>
                        <span class="underlined">{post.title.clone()}</span>
                      </A>
                    </h4>
                  </div>
                  <div class="overflow-hidden flex-grow p-4 max-h-48 text-ellipsis line-clamp-3">
                    <p class="mb-4 text-nord6">{post.content.clone()}</p>
                  </div>
                  <div class="p-4 mt-auto">
                    <p class="mb-2 text-sm text-nord3">
                      {post.tags.iter().map(|tag| view! { <Tag tag=tag.clone() /> }).collect::<Vec<_>>()}
                    </p>
                    <p class="text-sm text-nord3">"Created: " {formatted_date}</p>
                  </div>
                </div>
              }
            })
            .collect::<Vec<_>>()
        }}
      </div>
    </div>
  }
}
