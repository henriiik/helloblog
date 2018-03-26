#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate pulldown_cmark;
extern crate rocket;

use pulldown_cmark::Parser;
use pulldown_cmark::html::push_html;
use rocket::response::NamedFile;
use rocket::response::content::Html;
use rocket::response::status::NotFound;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> Result<Html<String>, NotFound<String>> {
    page(PathBuf::from("index"))
}

#[get("/static/<file..>")]
fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/<path..>", rank = 2)]
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

    Ok(Html(format!(
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>HelloBlog</title><link rel="stylesheet" type="text/css" href="/static/hello.css"><script type="text/javascript" src="/static/hello.js" defer></script></head><body><h1><a href="/">HelloBlog</a></h1>{}</body></html>"#,
        html_buf
    )))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, file, page])
        .launch();
}
