#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate rocket_contrib;
extern crate diesel;
extern crate mlib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate r2d2;
extern crate r2d2_diesel;

// Server Imports
// Used to Setup DB Pool
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::Status;

// Used for Routes
use rocket::Request;
use rocket::response::NamedFile;
use rocket_contrib::JSON;

// Std Imports
use std::path::{Path, PathBuf};

// DB Imports
use diesel::prelude::*;
use diesel::update;
use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection, GetTimeout};
use r2d2_diesel::ConnectionManager;
use mlib::models::*;
use mlib::*;

fn main() {
    // Put site last so that path collision tries others
    // first
    rocket::ignite()
        .mount("/", routes![
            count,
            count_update,
            public,
            keybase,
            static_files,
            index,
            site
        ])
        .launch();
}

// DB Items
lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
}

pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = GetTimeout;
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(e) => Failure((Status::InternalServerError, e)),
        }
    }
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

#[get("/keybase.txt")]
fn keybase() -> Option<NamedFile> {
    NamedFile::open("src/client/keybase.txt").ok()
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
fn count(db: DB) -> JSON<Clicks> {
    use mlib::schema::counts::dsl::*;
    let result = counts.first::<Count>(db.conn())
        .expect("Error loading clicks");

    JSON(Clicks {
        count: result.clicks,
    })
}

#[put("/count")]
fn count_update(db: DB) -> JSON<Clicks> {
    use mlib::schema::counts::dsl::*;
    let query = counts.first::<Count>(db.conn())
        .expect("Error loading clicks");
    let val = query.clicks + 1;

    update(counts.find(1))
        .set(clicks.eq(val))
        .execute(db.conn())
        .unwrap();

    JSON(Clicks {
        count: val,
    })
}

#[derive(Deserialize, Serialize)]
pub struct Clicks {
    pub count: i32,
}
