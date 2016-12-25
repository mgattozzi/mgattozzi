#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile>{
     NamedFile::open(Path::new("src/client/static").join(file)).ok()
}

#[get("/public/<file..>")]
fn public(file: PathBuf) -> Option<NamedFile>{
     NamedFile::open(Path::new("src/client/public").join(file)).ok()
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("src/client/index.html").ok()
}

#[get("/<path..>")]
fn site(path: PathBuf) -> Option<NamedFile> {
    let _ = path;
    NamedFile::open("src/client/index.html").ok()
}

fn main() {
    // Put site last so that path collision tries others
    // first
    rocket::ignite().mount("/", routes![public, static_files, index, site]).launch();
}
