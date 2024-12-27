use leptos::prelude::*;

#[component]
pub(crate) fn Blog() -> impl IntoView {
  let (count, set_count) = signal(0);

  view! {
    <div>
      <h2>Blog</h2>
      <p>Welcome to the blog!</p>
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
      </div>
    </div>
  }
}
