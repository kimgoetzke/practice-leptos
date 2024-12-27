use leptos::prelude::*;

fn main() {
  console_error_panic_hook::set_once();
  mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
  let (count, set_count) = signal(0);

  view! {
    <div class="h-full min-h-screen bg-background">
      <div class="grid grid-flow-row auto-rows-max justify-center justify-items-center items-center">
        <div>
          <button
            class="button-standard"
            // on:click will run whenever the `click` event fires
            // every event handler is defined as `on:{eventname}`

            // we're able to move `set_count` into the closure
            // because signals are Copy and 'static

            on:click=move |_| *set_count.write() += 1
          >
            // text nodes in RSX should be wrapped in quotes,
            // like a normal Rust string
            "Click me: "
            {count}
          </button>
          <p>
            <strong>"Reactive: "</strong>
            // you can insert Rust expressions as values in the DOM
            // by wrapping them in curly braces
            // if you pass in a function, it will reactively update
            {move || count.get()}
          </p>
          <p>
            <strong>"Reactive shorthand: "</strong>
            // you can use signals directly in the view, as a shorthand
            // for a function that just wraps the getter
            {count}
          </p>
          <p>
            <strong>"Not reactive: "</strong>
            // NOTE: if you just write {count.get()}, this will *not* be reactive
            // it simply gets the value of count once
            {count.get()}
          </p>
        </div>
      </div>
    </div>
  }
}
