# Practice WASM & Leptos



## How to develop

### Start

To start, run the following command and open `localhost:8080` in your browser:
```shell
trunk serve --open
```

### Using Nix Flakes, JetBrains RustRover & Direnv

You can run this project in any way you like, but I have set things up to make it easy to develop using JetBrains
RustRover. For this, you'll need:

- `direnv`
- Any Direnv integration plugin e.g. https://plugins.jetbrains.com/plugin/15285-direnv-integration
- `nix`

This way, you'll just need to `direnv allow` in the project directory after which all prerequisites (incl. Rust, Cargo,
all Bevy dependencies, etc.) will be available to you. The JetBrains plugin will ensure that the environment is
available to your IDE and you can run the project from there (vs `cargo build` and `cargo run` in the terminal).

### Using Nix Flakes

Without `direnv`, you can use the Nix Flake by running `nix develop` in the project directory. If you want to use an IDE
such as JetBrains RustRover, you'll have to set up the environment manually.

Upgrade the flake by running `nix flake update` in the repository's base directory.

### Reminders


#### Run configurations

- Create a run configuration with environment variable `RUST_LOG=procedural_generation_2=debug` for debug logs
- Create a run configuration with environment variable
  `RUST_LOG=procedural_generation_2=debug,procedural_generation_2::generation::object=trace` to add WFC trace logs too
