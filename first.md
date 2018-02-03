# First post!

* create project

  ```zsh
  ~/w/rust ❯❯❯ cargo new --bin helloblog
      Created binary (application) `helloblog` project
  ~/w/rust ❯❯❯ cd helloblog
  ~/w/r/helloblog ❯❯❯ code .
  ~/w/r/helloblog ❯❯❯
  ```

* initial commit
* create first.md
* prettier formats
* checkout rocket documentation https://rocket.rs/guide/getting-started/

- `cargo.toml`

  ```toml
  [dependencies]
  rocket = "0.3.6"
  rocket_codegen = "0.3.6"
  ```

- `src/main.rs`

  ```rust
  #![feature(plugin)]
  #![plugin(rocket_codegen)]

  extern crate rocket;

  #[get("/")]
  fn index() -> &'static str {
      "Hello, world!"
  }

  fn main() {
      rocket::ignite().mount("/", routes![index]).launch();
  }
  ```

- it works!
