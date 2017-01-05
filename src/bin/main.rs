#![feature(plugin, proc_macro, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate diesel;
extern crate mlib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// Server Imports
use rocket::response::NamedFile;
use rocket_contrib::JSON;

// Std Imports
use std::path::{Path, PathBuf};

// DB Imports
use diesel::prelude::*;
use diesel::update;
use mlib::models::*;
use mlib::*;


fn main() {
    // Put site last so that path collision tries others
    // first
    rocket::ignite().mount("/", routes![count, count_update, public, static_files, index, site]).launch();
}

// Routes
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

#[get("/count")]
fn count() -> JSON<Clicks> {
    use mlib::schema::counts::dsl::*;
    let connection = establish_connection();
    let result = counts.first::<Count>(&connection)
        .expect("Error loading clicks");

    JSON(Clicks {
        count: result.clicks,
    })
}

#[put("/count")]
fn count_update() -> JSON<Clicks> {
    use mlib::schema::counts::dsl::*;
    let connection = establish_connection();
    let query = counts.first::<Count>(&connection)
        .expect("Error loading clicks");
    let val = query.clicks + 1;

    update(counts.find(1))
        .set(clicks.eq(val))
        .execute(&connection)
        .unwrap();

    JSON(Clicks {
        count: val,
    })
}

#[derive(Deserialize, Serialize)]
pub struct Clicks {
    pub count: i32,
}
