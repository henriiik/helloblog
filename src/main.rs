#![feature(plugin)]
#![plugin(rocket_codegen)]

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

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
