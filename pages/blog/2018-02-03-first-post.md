## [First Post](/blog/2018-02-03-first-post)

2018-02-03 by Henrik Sjööh

I decided to create a blog, in rust, and to blog about creating it. It sounded like a fun thing to do. What if you could make your whole website in just markdown files. I suspect sooner or later something more will have to be added.

Another thought I have been mulling over lately is what if there was no css classes needed, everything "just worked". It would probably be pretty limited, but perhaps that could help creativity.

## Creating the project

Before creating the project, [rust had to be installed](https://www.rust-lang.org/en-US/install.html) of course. But I had already done that so it was time for `cargo new`. Exciting!

```sh
$ cargo new --bin helloblog
     Created binary (application) `helloblog` project
$ cd helloblog
$ cargo run
   Compiling helloblog v0.1.0 (file:///Users/henke/work/rust/helloblog)
    Finished dev [unoptimized + debuginfo] target(s) in 1.79 secs
     Running `target/debug/helloblog`
Hello, world!
$ code .
```

This generates a new rust project that when compiled creates a binary that prints hello world. Opened up the project in my editor of choice, [VSCode](https://code.visualstudio.com), and [committed the generated project](https://github.com/henriiik/helloblog/commit/e62ef3e71701114fa413459db9f531b8f2d2f6f2).

The next step was to create my first markdown file and start recording my progress, I called it `first.md`. In the process of doing this, I realized I have the [Prettier plugin](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode) for VSCode installed and that automatically formats my markdown, nice!

## Rocket

I had decided to use the [Rocket framework](https://rocket.rs) because it seemed to be forward thinking, using rusts powerful types eliminate whole classes of bugs.

So I headed over to Rocket's [getting started guide](https://rocket.rs/guide/getting-started/) and it told me that I had to add some stuff to my `cargo.toml`.

```toml
[dependencies]
rocket = "0.3.6"
rocket_codegen = "0.3.6"
```

Which I did. I also found this sweet snippet, that I replaced everything in my `src/main.rs` with.

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

And just like that, I had a web server. Wohoo!

```
$ cargo run
   Compiling libc v0.2.36
   Compiling lazy_static v0.2.11
   Compiling yansi v0.3.4
   Compiling matches v0.1.6
   Compiling ordermap v0.2.13
   Compiling gcc v0.3.54
   Compiling byteorder v1.2.1
   Compiling cfg-if v0.1.2
   Compiling httparse v1.2.4
   Compiling language-tags v0.2.2
   Compiling percent-encoding v1.0.1
   Compiling traitobject v0.1.0
   Compiling typeable v0.1.2
   Compiling serde v1.0.27
   Compiling safemem v0.2.0
   Compiling unicode-normalization v0.1.5
   Compiling untrusted v0.5.1
   Compiling either v1.4.0
   Compiling scopeguard v0.3.3
   Compiling state v0.3.3
   Compiling smallvec v0.4.4
   Compiling pear v0.0.12
   Compiling rayon-core v1.3.0
   Compiling version_check v0.1.3
   Compiling unicode-bidi v0.3.4
   Compiling time v0.1.39
   Compiling rand v0.3.20
   Compiling num_cpus v1.8.0
   Compiling memchr v1.0.2
   Compiling isatty v0.1.6
   Compiling log v0.4.1
   Compiling base64 v0.6.0
   Compiling coco v0.1.1
   Compiling unicase v1.4.2
   Compiling rocket_codegen v0.3.6
   Compiling pear_codegen v0.0.12
   Compiling rocket v0.3.6
   Compiling idna v0.1.4
   Compiling log v0.3.9
   Compiling mime v0.2.6
   Compiling rayon v0.7.1
   Compiling url v1.6.0
   Compiling ring v0.11.0
   Compiling hyper v0.10.13
   Compiling toml v0.4.5
   Compiling cookie v0.9.2
   Compiling helloblog v0.1.0 (file:///Users/henke/work/rust/helloblog)
    Finished dev [unoptimized + debuginfo] target(s) in 44.51 secs
     Running `target/debug/helloblog`
🔧  Configured for development.
    => address: localhost
    => port: 8000
    => log: normal
    => workers: 8
    => secret key: generated
    => limits: forms = 32KiB
    => tls: disabled
🛰  Mounting '/':
    => GET /
🚀  Rocket has launched from http://localhost:8000
GET / text/html:
    => Matched: GET /
    => Outcome: Success
    => Response succeeded.
```

I fired it up, and it works!

## Markdown

Next up, Markdown! I read somewhere that [`rustdoc` is being remade](https://github.com/steveklabnik/rustdoc), so they probably have some good library to handle it.

They do! It's called [pulldown-cmark](https://github.com/google/pulldown-cmark) and it's made by [a cool guy](http://levien.com) that also makes [a cool editor](https://github.com/google/xi-editor).

Anyway this is what goes in `cargo.toml`

```toml
pulldown-cmark = { version = "0.0.11", default-features = false }
```

At this point I feel that writing a blog while coding is weird. Probably because I am not used to this kind of writing. But in the end maybe the project will be better for me having to motivate choices.

Anyway I started digging into the `pulldown-cmark` docs and I did have some trouble figuring out how to use the library. But after much clicking around [i found a nice snippet](https://docs.rs/pulldown-cmark/0.1.0/pulldown_cmark/html/fn.push_html.html).

```rust
use pulldown_cmark::{html, Parser};

let markdown_str = r#"
hello
=====

* alpha
* beta
"#;
let parser = Parser::new(markdown_str);

let mut html_buf = String::new();
html::push_html(&mut html_buf, parser);

assert_eq!(html_buf, r#"<h1>hello</h1>
<ul>
<li>alpha</li>
<li>beta</li>
</ul>
"#);
```

I also realized that the latest version of the library is `0.1.0` which is not what it said in the readme.

This is the index function after integrating the markdown parser. It was fairly straightforward. I was afraid the would be complications with the return type. I my previous attempts at learning Rust I have had some trouble with return types. But changing the return type from `&'static str` to `String` worked, like magic!

Probably because `String` is heap allocated and so the caller can be given ownership of it. Rocket surely does something cool with this as well.

```rust
extern crate pulldown_cmark;
extern crate rocket;

use pulldown_cmark::{html, Parser};

#[get("/")]
fn index() -> String {
    let markdown_str = r#"
hello
=====
* alpha
* beta
"#;
    let parser = Parser::new(markdown_str);

    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    html_buf
}
```

## HTML

When looking the the page now the HTML tags are plain to see. Which lets me know that the markdown parsing is working, yay! But seeing the HTML tags is rarely what you want, rendered HTML is what I want.

I guessed that the problem was that there were no wrapping `<html></html>` tags. To fixed this i turned to the [`format!` macro](https://doc.rust-lang.org/std/macro.format.html).

It didn't work. The `<html>` tags were visible as well.

I started clicking around in the chrome dev tools, as I usually do when i don't know what is wrong. And i saw that the `Content-Type` header was being sent with the value `text/plain`. Which makes sense, since I am returning a `String`. But it's not what i wanted.

Rocket docs to the rescue! [You can wrap your response in a struct to get the `Content-Type` you want](https://rocket.rs/guide/responses/#wrapping), awesome! [rocket::response::content::Html](https://api.rocket.rs/rocket/response/content/struct.Html.html) is what we want.

```rust
extern crate pulldown_cmark;
extern crate rocket;

use rocket::response::content;
use pulldown_cmark::{html, Parser};

#[get("/")]
fn index() -> content::Html<String> {
    let markdown_str = r#"
hello
=====
* alpha
* beta
"#;
    let parser = Parser::new(markdown_str);

    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    content::Html(format!("{}", html_buf))
}
```

It works! Now page rendered correctly. (I forgot to remove the `format!` macro).

## File Handling

The last thing that needs doing before markdown-on-disk-to-html-in-browser is done, is to read the markdown from a file. I'm sure the rust book can help!

After some quick googling i found the chapter called ["Reading a file"](https://doc.rust-lang.org/book/second-edition/ch12-02-reading-a-file.html), sound good! And it does indeed contain a snippet that reads a file.

```rust
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // ...snip...
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

Fairly straight forward to integrate it into the `index` function.

```rust
extern crate pulldown_cmark;
extern crate rocket;

use std::fs::File;
use std::io::Read;
use rocket::response::content;
use pulldown_cmark::{html, Parser};

#[get("/")]
fn index() -> content::Html<String> {
    let mut f = File::open("first.md").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let parser = Parser::new(&contents);

    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    content::Html(format!("{}", html_buf))
}
```

I did not include `use std::io::Read` at first, and i got an error in VSCode that was not displayed correctly. But I ran the build command in the terminal to see the complete error and that helped!

And that was it! End to end! Disk to browser! SUCCESS!

## Finishing up

Errors were not being handled so i added that. Before the file opening and reading were using `.unwrap()` which would cause a panic at runtime if something went wrong. I changed `index` to return a result and i used the new (-ish) `?` syntax and it was great.

I selected the `NotFound` error since it matches if the file is not found. But i think it can be improved.

I also messed around with the `use` statements, to make it more clear what was happening.

To make the page a proper HTML5 document some more tags were needed as well.

```rust
extern crate pulldown_cmark;
extern crate rocket;

use std::fs::File;
use std::io::Read;
use rocket::response::content::Html;
use rocket::response::status::NotFound;
use pulldown_cmark::Parser;
use pulldown_cmark::html::push_html;

#[get("/")]
fn index() -> Result<Html<String>, NotFound<String>> {
    let mut f = File::open("first.md").map_err(|e| NotFound(format!("{}", e)))?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .map_err(|e| NotFound(format!("{}", e)))?;

    let parser = Parser::new(&contents);

    let mut html_buf = String::new();
    push_html(&mut html_buf, parser);

    Ok(Html(format!(
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><title>helloblog</title></head><body>{}</body></html>"#,
        html_buf
    )))
}
```

## What's Next

Whilst writing the post from my notes, some things became clear.

* I really need a spellchecker.
  * fortunately i found a good [spell check extension for VSCode](https://marketplace.visualstudio.com/items?itemName=ban.spellright) that uses the system spelling API.
* It's pretty boring without images.
* The page really needs some styles

Some un-answered questions came to mind as well

* How to deploy?
  * I guess stick all the files in a container and just run it?
* How to handle HTTPS?
  * Apparently rocket can do this, yay!

The questions were quickly answered.

Check out the next post in the series [Styles](/blog/2018-02-11-pages).
