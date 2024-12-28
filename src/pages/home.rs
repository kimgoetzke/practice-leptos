use leptos::prelude::*;
use std::time::Duration;

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
    <div class="centered-layout">
      <h1>Welcome!</h1>
      <div class="p-4"></div>
      <div class="typewriter">{' '}<p>{move || current_text.get()}</p></div>
    </div>
  }
}

fn animate_text(text: Vec<String>, set_current_text: WriteSignal<String>) {
  let (current_index, set_current_index) = signal(0);
  let (char_index, set_char_index) = signal(0);
  let (is_writing, set_is_writing) = signal(true);
  let (is_waiting, set_is_waiting) = signal(false);

  set_interval(
    {
      let text = text.clone();
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
            wait(4, set_is_writing, set_is_waiting);
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
      }
    },
    Duration::from_millis(75),
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
