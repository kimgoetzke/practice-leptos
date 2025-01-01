use leptos::logging::log;
use leptos::prelude::*;
use leptos::web_sys::js_sys::Date;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_params_map;
use std::sync::LazyLock;

#[derive(Clone, PartialEq)]
pub(crate) struct BlogPost {
  pub(crate) id: String,
  pub(crate) title: String,
  pub(crate) content: String,
  pub(crate) tags: Vec<String>,
  pub(crate) created_at: f64,
  pub(crate) image: Option<String>,
}

pub(crate) static BLOG_POSTS: LazyLock<Vec<BlogPost>> = LazyLock::new(|| create_test_blog_posts());

pub fn create_test_blog_posts() -> Vec<BlogPost> {
  vec![
    BlogPost {
      id: "1".to_string(),
      title: "Hello, world!".to_string(),
      content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string(),
      tags: vec!["hello".to_string(), "world".to_string()],
      created_at: Date::now(),
      image: Some("/images/image1.png".to_string()),
    },
    BlogPost {
      id: "2".to_string(),
      title: "Another post".to_string(),
      content: "This is another blog post.".to_string(),
      tags: vec!["another".to_string(), "post".to_string()],
      created_at: Date::now(),
      image: Some("/images/image2.png".to_string()),
    },
    BlogPost {
      id: "3".to_string(),
      title: "And the third one".to_string(),
      content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string(),
      tags: vec!["hello".to_string(), "world".to_string()],
      created_at: Date::now(),
      image: None,
    },
    BlogPost {
      id: "4".to_string(),
      title: "Post #4".to_string(),
      content: "This is a short blog post.".to_string(),
      tags: vec!["hello".to_string(), "world".to_string()],
      created_at: Date::now(),
      image: Some("/images/image3.png".to_string()),
    },
  ]
}

#[component]
pub(crate) fn Post() -> impl IntoView {
  let params = use_params_map();
  let id = move || params.read().get("id").unwrap_or_default();
  log!("id: {}", id());
  let post_signal = Memo::new(move |_| {
    BLOG_POSTS.iter().find(|post| post.id == id()).cloned()
  });
  view! {
    <Show
      when=move || post_signal.get().is_some()
      fallback=|| {
        view! {
          <div class="centered-layout">
            <h1>"404"</h1>
            <div class="py-6"></div>
            <p>"Sorry, I didn't find this blog post."</p>
          </div>
        }
      }
    >
      {move || {
        let ref post = post_signal.get().expect("Post not found");
        view! {
          <div>
            {post
              .image
              .as_ref()
              .map(|img| view! { <img class="float-right pt-2 mx-auto rounded-xl" src=img.clone() /> })}
            <h2>{post.title.clone()}</h2> <p>{post.content.clone()}</p> <p class="pt-6 text-sm text-nord3">
              <strong>"Tags: "</strong>
              {post.tags.join(", ")}
            </p> <p class="text-sm text-nord3">
              <strong>"Created at: "</strong>
              {post.created_at}
            </p>
          </div>
        }
      }}
    </Show>
  }
}
