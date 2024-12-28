use leptos::attr::Datetime;
use leptos::prelude::*;
use leptos::web_sys::js_sys::Date;

struct BlogPost {
  title: String,
  content: String,
  tags: Vec<String>,
  created_at: f64,
}

#[component]
pub(crate) fn Blog() -> impl IntoView {
  let posts = vec![
    BlogPost {
      title: "Hello, world!".to_string(),
      content: "This is a blog post.".to_string(),
      tags: vec!["hello".to_string(), "world".to_string()],
      created_at: Date::now(),
    },
    BlogPost {
      title: "Another post".to_string(),
      content: "This is another blog post.".to_string(),
      tags: vec!["another".to_string(), "post".to_string()],
      created_at: Date::now(),
    },
  ];
  let (count, set_count) = signal(0);

  view! {
    <div>
      <h2>Blog</h2>
      <p class="pb-6">Welcome to the blog!</p>

      <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
        {move || {
          posts
            .iter()
            .map(|post| {
              view! {
                <div class="p-4 rounded-lg shadow-md transition hover:shadow-lg bg-nord6">
                  <h4 class="mb-2 text-xl font-semibold text-nord0">{post.title.clone()}</h4>
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
