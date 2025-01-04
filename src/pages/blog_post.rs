use leptos::logging::log;
use leptos::prelude::*;
use leptos::web_sys::js_sys::Date;
use leptos::{component, view, IntoView};
use leptos_router::hooks::use_params_map;
use std::sync::LazyLock;
use leptos::wasm_bindgen::JsValue;
use crate::components::*;

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
      title: "Lorem ipsum dolor sit amet".to_string(),
      content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string(),
      tags: vec!["elementum".to_string(), "ipsum".to_string()],
      created_at: Date::now(),
      image: Some("/images/image-1.png".to_string()),
    },
    BlogPost {
      id: "2".to_string(),
      title: "Commodo mauris cubilia himenaeos; fringilla ligula dolor ullamcorper nisl euismod".to_string(),
      content: "Eleifend sapien cras iaculis non erat mus non? Enim nec tempus pellentesque neque dignissim viverra nam porta.".to_string(),
      tags: vec!["dolor".to_string(), "elementum".to_string()],
      created_at: Date::now(),
      image: Some("/images/image-2.png".to_string()),
    },
    BlogPost {
      id: "3".to_string(),
      title: "Eleifend sapien".to_string(),
      content: "Potenti feugiat viverra urna auctor tincidunt lacinia elementum elementum. Duis nam vulputate adipiscing maximus accumsan a. Velit mi per primis ex ligula posuere. Malesuada magnis nam litora donec hac in platea. Ac eleifend sodales metus ut netus tellus. Vitae dolor vel cubilia ornare praesent enim conubia. Faucibus dis habitasse vehicula dui habitasse fames habitasse curae. Platea in pulvinar libero aliquam odio. Lorem est bibendum eleifend maecenas a ut facilisis suscipit ut.".to_string(),
      tags: vec!["elementum".to_string(), "ipsum".to_string()],
      created_at: Date::now(),
      image: Some("/images/image-3.png".to_string()),
    },
    BlogPost {
      id: "4".to_string(),
      title: "Facilisis rhoncus molestie nascetur; elit vel aenean".to_string(),
      content: "Etiam odio porta lectus phasellus magnis tempor pellentesque.".to_string(),
      tags: vec!["elementum".to_string(), "tempor".to_string()],
      created_at: Date::now(),
      image: Some("/images/image-4.png".to_string()),
    },
    BlogPost {
      id: "5".to_string(),
      title: "Laoreet facilisis senectus".to_string(),
      content: "Risus luctus aenean dui; felis tellus ornare. Sapien pellentesque molestie consequat suscipit erat vel curae rutrum curabitur. Nostra etiam fermentum aenean dignissim etiam fermentum non. In feugiat nascetur senectus per in mattis cursus metus. Taciti lacinia dolor pellentesque sem facilisi ipsum. Curabitur maximus vulputate lorem pharetra lobortis. Mattis dapibus purus, elementum curabitur posuere quis pellentesque dignissim. Dignissim massa tristique orci sodales vestibulum.".to_string(),
      tags: vec!["sapien".to_string(), "ipsum".to_string()],
      created_at: Date::now(),
      image: None,
    },
    BlogPost {
      id: "6".to_string(),
      title: "Fringilla massa inceptos ad etiam cubilia dapibus vulputate finibus bibendum".to_string(),
      content: "Commodo mauris cubilia himenaeos; fringilla ligula dolor ullamcorper nisl euismod. Imperdiet parturient duis ac nam laoreet magna molestie. Hac in parturient id euismod ultricies metus hendrerit class. Magnis nostra facilisi ad est, maximus vestibulum laoreet. Parturient in commodo magna lacus platea cubilia suspendisse. Est elit fermentum odio dictum eleifend sodales fermentum natoque.".to_string(),
      tags: vec!["elementum".to_string(), "sapien".to_string()],
      created_at: Date::now(),
      image: Some("/images/image-6.png".to_string()),
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
        let date = Date::new(&JsValue::from_f64(post.created_at));
        let formatted_date = format!("{}", date.to_locale_date_string("en-GB", &JsValue::undefined()));
        view! {
          <div>
            <div class="flex overflow-hidden justify-center items-center mb-6 max-h-96 rounded-xl">
              {post.image.as_ref().map(|img| view! { <img class="" src=img.clone() /> })}
            </div>
            <h2>{post.title.clone()}</h2>
            <p>{post.content.clone()}</p>
            <p class="py-6 text-sm text-nord3">
              {post.tags.iter().map(|tag| view! { <Tag tag=tag.clone() /> }).collect::<Vec<_>>()}
            </p>
            <p class="text-sm text-nord3">
              <strong>"Created at: "</strong>
              {formatted_date}
            </p>
          </div>
        }
      }}
    </Show>
  }
}
