#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate comrak;
extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate mlib;
extern crate serde_json;
extern crate tera;

use comrak::{ markdown_to_html, ComrakOptions };
// Server Imports
// Used to Setup DB Pool
use rocket::request::{ Outcome, FromRequest };
use rocket::Outcome::{ Success, Failure };
use rocket::http::Status;

// Used for Routes
use rocket::Request;
use rocket::response::NamedFile;
use rocket_contrib::{ Template, Json };

// Std Imports
use std::path::{ Path, PathBuf };
use std::io::{ BufReader, Read };

// DB Imports
use diesel::prelude::*;
use diesel::update;
use diesel::pg::PgConnection;
use r2d2::{ Pool, PooledConnection, GetTimeout };
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
            keybase,
            static_files,
            index,
            posts
        ])
        .attach(Template::fairing())
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
#[get("/static/<file..>", rank=1)]
fn static_files(file: PathBuf) -> Option<NamedFile>{
    NamedFile::open(Path::new("static").join(file)).ok()
}

#[get("/<posts..>", rank=2)]
fn posts(mut posts: PathBuf) -> Option<Template> {
    posts.set_extension("md");
    let path = Path::new("posts").join(&posts);

    let mut buffer = String::new();
    let file = NamedFile::open(&path);

    match file {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            match reader.read_to_string(&mut buffer) {
                Ok(_) => {
                    let post = Post::new(markdown_to_html(&buffer, &ComrakOptions::default()));
                    Some(Template::render("post", post))
                },
                Err(_) => None
            }
        },
        Err(_) => {
            posts.set_extension("");
            let nav = posts.to_str().unwrap().clone();
            match nav {
                "about"    => Some(Template::render("about",    Nav::new(1))),
                "archive"  => Some(Template::render("archive",  Nav::new(2))),
                "contact"  => Some(Template::render("contact",  Nav::new(3))),
                "resume"   => Some(Template::render("resume",   Nav::new(4))),
                "counting" => Some(Template::render("counting", Nav::new(5))),
                _          => None,
            }
        },
    }
}

#[get("/keybase.txt")]
fn keybase() -> Option<NamedFile> {
    NamedFile::open("keybase.txt").ok()
}

#[get("/")]
fn index() -> Option<Template> {
    Some(Template::render("index", Nav::new(0)))
}

#[get("/count")]
fn count(db: DB) -> Json<Clicks> {
    use mlib::schema::counts::dsl::*;
    let result = counts.first::<Count>(db.conn())
        .expect("Error loading clicks");

    Json(Clicks {
        count: result.clicks,
    })
}

#[put("/count")]
fn count_update(db: DB) -> Json<Clicks> {
    use mlib::schema::counts::dsl::*;
    let query = counts.first::<Count>(db.conn())
        .expect("Error loading clicks");
    let val = query.clicks + 1;

    update(counts.find(1))
        .set(clicks.eq(val))
        .execute(db.conn())
        .unwrap();

    Json(Clicks {
        count: val,
    })
}

#[derive(Deserialize, Serialize)]
pub struct Nav {
    url: i32,
}

impl Nav {
    pub fn new(url: i32) -> Self {
        Self { url }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Clicks {
    pub count: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Post {
    data: String,
    url: i32,
}

impl Post {
    pub fn new(data: String) -> Self {
        Self { data, url: -1 }
    }
}
