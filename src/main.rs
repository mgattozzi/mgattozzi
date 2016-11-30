extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate clap;
extern crate crypto;
extern crate spongedown;
extern crate toml;

mod md;
mod util;
mod css;
mod update;
mod mount_site;
mod config;

// Command Argument Imports
use clap::{App, Arg};

// Web Library Imports
use iron::Iron;
use mount::Mount;

// Standard Library Imports
use std::fs::create_dir;

// Binary Function Imports
use util::{mkpath,exists};
use css::compile_css;
use update::file_updater;
use mount_site::mount_dirs;
use config::{parse_config, css, Config};

/// Setup webserver then launch it
fn main() {

    let args = App::new("static")
        .version("0.1.0")
        .author("Michael Gattozzi <mgattozzi@gmail.com>")
        .about("Personal website")
        .get_matches();

    if let Some(config) = parse_config() {

        Iron::new(setup(config)).http("127.0.0.1:3000").unwrap();

    } else {

        println!("Failed to parse configuration. Make sure your Site.toml file is correct.");

    }
}

fn setup(conf: Config) -> Mount {
    println!("Setting up server");

    if !exists(&mkpath("site")) {
        // site doesn't exist yet so create it
        let _ = create_dir("site");
    }

    let mut mount = Mount::new();

    if let Some(c) = css(&conf) {
        compile_css(&c);
    }

    mount_dirs(&mut mount);

    file_updater(&conf);

    println!("Server now running");

    mount
}
