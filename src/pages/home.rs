use crate::components::*;
use leptos::prelude::*;
use leptos::web_sys;
use leptos_use::{use_interval_fn, use_window_scroll};
use std::time::Duration;

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
  update_has_scrolled(set_has_scrolled);

  let (current_text, set_current_text) = signal(String::new());
  animate_text(text, set_current_text);

  view! {
    <div class="home-container">
      <section class="-mt-16 full-height-section">
        <h1>"Welcome!"</h1>
        <div class="speech-bubble">
          <div class="typewriter">{' '}<p class="font-retro">{move || current_text.get()}</p></div>
        </div>
        <div>
          <img src="/images/avatar-1.gif" class="w-52 h-52 rounded-full drop-shadow-xl" />
        </div>
        <Show when=move || !has_scrolled.get() fallback=|| ()>
          <div class="bouncing-arrow hover:bg-nord1" on:click=move |_| scroll_down()>
            <lucide_leptos::ArrowDown size=48 />
          </div>
        </Show>
      </section>
      <section class="content-section">
        <h2>"I've been working as a web developer in financial technology"</h2>
        <p class="text-xl font-retro">
          "My focus has been on "<span class="highlighted">"authentication- and authorisation"</span>"-related topics"
          ", such as single sign-on (SSO), user provisioning, single logout (SLO), JWT or device deny listing, and permissions. I mostly work with "
          <span class="highlighted">Java</span>", various "<span class="highlighted">AWS services</span>
          ", and occasionally with "<span class="highlighted">"TypeScript/NextJS"</span>"."
        </p>
        <Timeline>
          <TimelineEntry
            date="Jan 2025".to_string()
            sub_title="Goji Investments".to_string()
            title="Senior Software Engineer".to_string()
            url=Some("https://www.goji.investments/".to_string())
            bullet_points=vec![
              "Goji is financial technology company, providing a white label investment platform to service and distributing private funds at scale"
                .to_string(),
            ]
          />
          <TimelineEntry
            date="Jan 2024".to_string()
            sub_title="Goji Investments".to_string()
            title="Software Engineer".to_string()
            url=Some("https://www.goji.investments/".to_string())
            bullet_points=vec![
              "Independently designed and implemented an automated, daily validation mechanism for authentication-related certificates (first and third party) that checks upcoming expiration dates and integrates with alerting and communication tools to create alerts of varying severity"
                .to_string(),
              "Worked on various continuous improvements to Goji's proprietary permission system and deprecated legacy role-based logic which involved several sensitive migrations and data fixes"
                .to_string(),
              "Improved Goji's SSO implementation by proposing and implementing a mechanism that gives customers the ability to provision users to entities and integrate with Goji's permission logic via the their own identity provider"
                .to_string(),
              "Independently designed, proposed, implemented, and documented a SAML-based SLO flow".to_string(),
              "Worked with product, operations, and technical stakeholders to create detailed technical as well as customer-facing documentation for SSO and user provisioning"
                .to_string(),
              "Communicated with technical/product/executive-level staff of customers implementing SSO and SLO with Goji"
                .to_string(),
            ]
          />
          <TimelineEntry
            date="Mar 2023".to_string()
            sub_title="Goji Investments".to_string()
            title="Junior Software Engineer".to_string()
            url=Some("https://www.goji.investments/".to_string())
            bullet_points=vec![
              "Worked with a senior developer and the cloud operations lead to design and run an SSO proof of concept using OAuth/OIDC and AWS Cognito"
                .to_string(),
              "Designed, proposed, implemented, and documented SAML-based SSO for the customer and service logins across Goji's core platforms together with a senior developer, pair programming for large parts of the project"
                .to_string(),
              "Communicated directly with product/technical staff of customers using SSO during testing/implementation phase"
                .to_string(),
            ]
          />
        </Timeline>
      </section>
      <section class="content-section">
        <h2 class="text-right">"Previously, I was a startup executive"</h2>
        <a href="https://uk.linkedin.com/in/kimgoetzke" rel="external" target="tab" class="flex justify-end">
          <div class="inline-flex items-center space-x-5 text-right default-button">
            <span>Read more on</span>
            <lucide_leptos::Linkedin />
          </div>
        </a>
        <Timeline>
          <TimelineEntry
            date="Jul 2021 - Apr 2023".to_string()
            sub_title="Blissgrowth".to_string()
            title="Partner and Co-founder".to_string()
            url=Some("https://www.blissgrowth.com/".to_string())
            bullet_points=Vec::new()
          />
          <TimelineEntry
            date="May 2021 - Sep 2022".to_string()
            sub_title="Screenloop".to_string()
            title="Interim COO/CFO".to_string()
            url=Some("https://www.screenloop.com/".to_string())
            bullet_points=Vec::new()
          />
          <TimelineEntry
            date="Dec 2019 - May 2021".to_string()
            sub_title="Growth Street".to_string()
            title="COO and Executive Director (SMF3)".to_string()
            url=None
            bullet_points=Vec::new()
          />
          <TimelineEntry
            date="Feb 2017 - Dec 2019".to_string()
            sub_title="Growth Street".to_string()
            title="Operations Director".to_string()
            url=None
            bullet_points=Vec::new()
          />
        </Timeline>
      </section>
      <section class="content-section">
        <h2>
          "What I hope you take away from reading this page is that I am ambitious, I learn quickly, and I love software development"
        </h2>
        <h3 class="text-center">"From web development..."</h3>
        <Showcase
          image_path="images/image-2.png".to_string()
          description="A basic microservice architecture, written in Java using the Spring Framework, Postgres, Keycloak, etc."
            .to_string()
          link="https://github.com/kimgoetzke/practice-microservices".to_string()
        />
        <h3 class="text-center">"...to procedural generation..."</h3>
        <Showcase
          image_path="images/procedural-generation-1.png".to_string()
          description="A procedurally generated 2D, pixel art, tile set-based world, written in Rust using Bevy Engine"
            .to_string()
          link="https://github.com/kimgoetzke/procedural-generation-2".to_string()
        />
        <h3 class="text-center">"...to mobile development..."</h3>
        <Showcase
          image_path="/images/mobile-development-1.png".to_string()
          description="A minimalist list app for Android which I still use every day, written in C# using .NET MAUI and MongoDB Atlas and Realm"
            .to_string()
          link="https://github.com/kimgoetzke/listem".to_string()
        />
        <h3 class="text-center">"...to game development..."</h3>
        <Showcase
          image_path="images/game-development-1.png".to_string()
          description="A proof-of-concept for 3D pixel art, action RPG using 2D sprites, developed in C# and Unity"
            .to_string()
          link="https://github.com/kimgoetzke/game-muffin".to_string()
        />
        <h3 class="text-center">"...and much more!"</h3>
      </section>
    </div>
  }
}

fn update_has_scrolled(set_has_scrolled: WriteSignal<bool>) {
  let (_, y) = use_window_scroll();
  Effect::new(move |_| {
    set_has_scrolled.set(y.get() > 10.0);
  });
}

fn scroll_down() {
  let window = web_sys::window().unwrap();
  let options = web_sys::ScrollToOptions::new();
  options.set_top(window.inner_height().unwrap().as_f64().unwrap());
  options.set_behavior(web_sys::ScrollBehavior::Smooth);
  window.scroll_with_scroll_to_options(&options);
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
