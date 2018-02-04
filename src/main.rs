#![feature(plugin)]
#![plugin(rocket_codegen)]

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

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
