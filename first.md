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

## Rocket

* `cargo.toml`

  ```toml
  [dependencies]
  rocket = "0.3.6"
  rocket_codegen = "0.3.6"
  ```

* `src/main.rs`

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

* it works!

## Markdown

* render markdown?

* checkout `rustdoc`

* checkout `pulldown-cmark`

  * Raph Levien is cool

* `cargo.toml`

  ```toml
  pulldown-cmark = { version = "0.0.11", default-features = false }
  ```

* writing a blog while coding is weird

* `pulldown-cmark` docs hard, need getting started guide

  * https://docs.rs/pulldown-cmark/0.1.0/pulldown_cmark/struct.Parser.html

  - newest version is `0.1.0`
  - found guide https://docs.rs/pulldown-cmark/0.1.0/pulldown_cmark/html/fn.push_html.html

* classic problem return string from function

  * still cant remember

  - perhaps rocket can help

  - just setting return type to `String` works! magic!

## HTML

* page not rendered correctly, needs html tags

- format macro to the rescue!

  * https://doc.rust-lang.org/std/macro.format.html

- forgot rebuild

- html tags not really problem? rocket content type to the resque
  * https://rocket.rs/guide/responses/#wrapping
  * https://api.rocket.rs/rocket/response/content/struct.Html.html

## File Handling

* reading a file!

  * https://doc.rust-lang.org/book/second-edition/ch12-02-reading-a-file.html

* some error text missing from vscode

  * recompile to get full error

* fixed with
  ```rust
  use std::io::Read;
  ```

## Finishing up

* added some html tags to the template
* added some errror handling
  * https://rocket.rs/guide/responses/#result
