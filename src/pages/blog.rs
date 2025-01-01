use leptos::prelude::*;
use leptos_router::components::*;

#[component]
pub(crate) fn Blog() -> impl IntoView {
  let (count, set_count) = signal(0);

  view! {
    <div>
      <h2>Blog</h2>
      <p class="pb-6">Welcome to the blog!</p>
      <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
        {move || {
          let posts = crate::pages::blog_post::BLOG_POSTS.clone();
          posts
            .into_iter()
            .map(|post| {
              view! {
                <div class="p-4 rounded-lg shadow-md transition hover:shadow-lg bg-nord6">
                  <h4 class="mb-2 text-xl font-semibold text-nord0">
                    <A href=post.id.clone()>{post.title.clone()}</A>
                  </h4>
                  {move || {
                    post.image.as_ref().map(|img| view! { <img class="w-auto max-h-32 rounded-xl" src=img.clone() /> })
                  }}
                  <p class="mb-4 text-nord0">{post.content.clone()}</p>
                  <p class="mb-2 text-sm text-nord2">
                    <strong>"Tags: "</strong>
                    {post.tags.join(", ")}
                  </p>
                  <p class="text-sm text-nord3">
                    <strong>"Created at: "</strong>
                    {post.created_at}
                  </p>
                </div>
              }
            })
            .collect::<Vec<_>>()
        }}
      </div>
      <div class="py-6">
        <button class="button-standard" on:click=move |_| *set_count.write() += 1>
          "Subscribe"
        </button>
        <p>"Subscribers: "{count}</p>
      </div>
    </div>
  }
}
