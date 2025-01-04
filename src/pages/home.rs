use leptos::prelude::*;
use std::time::Duration;
use leptos_use::use_interval_fn;
use crate::components::*;

#[component]
pub(crate) fn Home() -> impl IntoView {
  let text = vec![
    "Thanks for visiting.",
    "It's great to see you here.",
    "How are you today?",
    "I hope you're happy!",
    "Seen the menu above?",
    "Don't be shy, click around!",
  ]
  .into_iter()
  .map(String::from)
  .collect::<Vec<String>>();
  let (current_text, set_current_text) = signal(String::new());
  animate_text(text, set_current_text);

  view! {
    <div class="home-container">
      <section class="full-height-section">
        <h1>"Welcome!"</h1>
        <div class="p-4"></div>
        <div class="typewriter">{' '}<p class="font-retro">{move || current_text.get()}</p></div>
        <div class="bouncing-arrow">
          <lucide_leptos::ArrowDown color="#88C0D0" size=48 />
        </div>
      </section>
      <section class="content-section">
        <h2>"Work experience"</h2>
        <Timeline />
      </section>
    </div>
  }
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

#[component]
pub(crate) fn Timeline() -> impl IntoView {
  view! {
    <div class="relative py-24 my-12 space-y-8 before:absolute before:inset-0 before:ml-5 before:-translate-x-px before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-nord7 before:to-transparent md:before:ml-[8.75rem] md:before:translate-x-0">
      <TimelineEntry
        date="Jan 2024".to_string()
        sub_title="Goji Investments".to_string()
        title="Senior Software Engineer".to_string()
        description="Goji is financial technology company, providing a white label investment platform to service and distributing private funds at scale."
          .to_string()
        icon=view! { <lucide_leptos::Rabbit color="#8FBCBB" /> }
      />
      <TimelineEntry
        date="Jan 2023".to_string()
        sub_title="Goji Investments".to_string()
        title="Software Engineer".to_string()
        description="Goji is financial technology company, providing a white label investment platform to service and distributing private funds at scale."
          .to_string()
        icon=view! { <lucide_leptos::Rat color="#8FBCBB" /> }
      />
      <TimelineEntry
        date="Feb 2023".to_string()
        sub_title="Goji Investments".to_string()
        title="Junior Software Engineer".to_string()
        description="Goji is financial technology company, providing a white label investment platform to service and distributing private funds at scale."
          .to_string()
        icon=view! { <lucide_leptos::Snail color="#8FBCBB" /> }
      />
    </div>
  }
}
