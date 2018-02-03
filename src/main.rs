#![feature(plugin)]
#![plugin(rocket_codegen)]

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

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
