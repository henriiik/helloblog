#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate pulldown_cmark;
extern crate rocket;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use rocket::response::content::Html;
use rocket::response::status::NotFound;
use pulldown_cmark::Parser;
use pulldown_cmark::html::push_html;

#[get("/")]
fn index() -> Result<Html<String>, NotFound<String>> {
    page(PathBuf::from("index"))
}

#[get("/<path..>")]
fn page(mut path: PathBuf) -> Result<Html<String>, NotFound<String>> {
    path.set_extension("md");

    let mut md_contents = String::new();
    File::open(Path::new("pages/").join(path))
        .map_err(|e| NotFound(format!("{}", e)))?
        .read_to_string(&mut md_contents)
        .map_err(|e| NotFound(format!("{}", e)))?;

    let parser = Parser::new(&md_contents);

    let mut html_buf = String::new();
    push_html(&mut html_buf, parser);

    let mut css_contents = String::new();
    File::open("hello.css")
        .map_err(|e| NotFound(format!("{}", e)))?
        .read_to_string(&mut css_contents)
        .map_err(|e| NotFound(format!("{}", e)))?;

    Ok(Html(format!(
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>HelloBlog</title><style>{}</style></head><body><h1><a href="/">HelloBlog</a></h1>{}</body></html>"#,
        css_contents,
        html_buf
    )))
}

fn main() {
    rocket::ignite().mount("/", routes![index, page]).launch();
}
