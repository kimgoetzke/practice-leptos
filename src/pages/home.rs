use leptos::prelude::*;
use std::time::Duration;
use leptos::wasm_bindgen::prelude::Closure;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::Event;
use leptos_use::use_interval_fn;
use crate::components::*;

#[component]
pub(crate) fn Home() -> impl IntoView {
  let text = vec![
    "Thanks for visiting.",
    "It's great to see you here.",
    "This website is about me, Kim Goetzke.",
    "I'm a London-based software developer.",
    "Scroll down to learn more about me.",
    "This website is built in Rust...",
    "...using Leptos, a Rust web framework.",
  ]
  .into_iter()
  .map(String::from)
  .collect::<Vec<String>>();
  let (has_scrolled, set_has_scrolled) = signal(false);
  let (current_text, set_current_text) = signal(String::new());

  update_has_scrolled(set_has_scrolled);
  animate_text(text, set_current_text);

  view! {
    <div class="home-container">
      <section class="full-height-section">
        <h1>"Welcome!"</h1>
        <div class="p-4"></div>
        <div class="speech-bubble">
          <div class="typewriter">{' '}<p class="font-retro">{move || current_text.get()}</p></div>
        </div>
        <div>
          <img src="/images/avatar-1.gif" class="w-52 h-52 rounded-full drop-shadow-xl" />
        </div>
        <Show when=move || !has_scrolled.get() fallback=|| ()>
          <div class="bouncing-arrow">
            <lucide_leptos::ArrowDown color="#88C0D0" size=48 />
          </div>
        </Show>
      </section>
      <section class="content-section">
        <h2>"Professionally, I've been working as a web developer"</h2>
        <Timeline>
          <TimelineEntry
            date="Jan 2025".to_string()
            sub_title="Goji Investments".to_string()
            title="Senior Software Engineer".to_string()
            bullet_points=vec![
              "Goji is financial technology company, providing a white label investment platform to service and distributing private funds at scale."
                .to_string(),
            ]
            icon=view! { <lucide_leptos::Rabbit color="#8FBCBB" /> }
          />
          <TimelineEntry
            date="Jan 2024".to_string()
            sub_title="Goji Investments".to_string()
            title="Software Engineer".to_string()
            bullet_points=vec![
              "I did some impressive stuff".to_string(),
              "Look at me, bro".to_string(),
              "I am amazing".to_string(),
            ]
            icon=view! { <lucide_leptos::Rat color="#8FBCBB" /> }
          />
          <TimelineEntry
            date="Mar 2023".to_string()
            sub_title="Goji Investments".to_string()
            title="Junior Software Engineer".to_string()
            bullet_points=vec![
              "I did some stuff".to_string(),
              "I also did X".to_string(),
              "And I learnt how to do Y".to_string(),
            ]
            icon=view! { <lucide_leptos::Snail color="#8FBCBB" /> }
          />
        </Timeline>
      </section>
      <section class="content-section">
        <h2 class="text-right">"...and before that I was a startup executive..."</h2>
      </section>
      <section class="content-section">
        <h2>"...but ever since changing my career, I have been in love with software development"</h2>
        <h3 class="text-center">"...from procedural generation..."</h3>
        <h3 class="text-center">"...to mobile development..."</h3>
        <h3 class="text-center">"...to game development..."</h3>
        <h3 class="text-center">"...and much more!"</h3>
      </section>
    </div>
  }
}

fn update_has_scrolled(set_has_scrolled: WriteSignal<bool>) {
  Effect::new(move |_| {
    let on_scroll = move |_| {
      let scroll_y = window().scroll_y().unwrap_or(0.0);
      set_has_scrolled.set(scroll_y > 10.0);
    };

    let closure = Closure::wrap(Box::new(on_scroll) as Box<dyn FnMut(Event)>);
    window()
      .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
      .unwrap();
    closure.forget();
  });
}

fn animate_text(text: Vec<String>, set_current_text: WriteSignal<String>) {
  let (current_index, set_current_index) = signal(0);
  let (char_index, set_char_index) = signal(0);
  let (is_writing, set_is_writing) = signal(true);
  let (is_waiting, set_is_waiting) = signal(false);

  use_interval_fn(
    move || {
      if is_waiting.get() {
        return;
      }

      let idx = current_index.get();
      let c_idx = char_index.get();
      let writing = is_writing.get();

      if writing {
        if c_idx < text[idx].len() {
          type_character(set_current_text, set_char_index, &text, idx, c_idx);
        } else {
          wait(2, set_is_writing, set_is_waiting);
        }
      } else {
        if c_idx > 0 {
          set_char_index.set(c_idx - 1);
          set_current_text.set(text[idx][..c_idx - 1].to_string());
        } else {
          set_current_index.set((idx + 1) % text.len());
          set_is_writing.set(true);
        }
      }
    },
    50,
  );
}

fn type_character(
  set_current_text: WriteSignal<String>,
  set_char_index: WriteSignal<usize>,
  text: &Vec<String>,
  text_index: usize,
  character_index: usize,
) {
  set_current_text.set(text[text_index][..=character_index].to_string());
  set_char_index.set(character_index + 1);
}

fn wait(seconds: u64, set_is_writing: WriteSignal<bool>, set_is_waiting: WriteSignal<bool>) {
  set_is_waiting.set(true);
  set_timeout(
    {
      let set_is_waiting = set_is_waiting.clone();
      move || set_is_waiting.set(false)
    },
    Duration::from_secs(seconds),
  );
  set_is_writing.set(false);
}