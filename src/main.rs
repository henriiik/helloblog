#![feature(plugin)]
#![plugin(rocket_codegen)]

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

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
