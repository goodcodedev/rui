#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use rocket::response::content;
use rocket::response::Response;
use rocket::http::ContentType;
use std::fs::File;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("../index.html")).ok()
}

#[get("/<file..>")]
fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("../").join(file)).ok()
}

#[get("/main.wasm")]
fn wasm<'a>() -> rocket::Response<'a> {
    Response::build()
        .header(ContentType::Binary)
        .sized_body(NamedFile::open("../main.wasm").ok().unwrap())
        .finalize()
}

fn main() {
    rocket::ignite().mount("/", routes![index, wasm, file]).launch();
}