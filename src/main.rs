extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate clap;
extern crate crypto;
extern crate spongedown;
extern crate toml;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_atomic;
extern crate logger;
extern crate router;

mod md;
mod util;
mod css;
mod update;
mod mount_site;
mod config;

// Loggin
use slog::{Logger, DrainExt};
use slog_atomic::*;

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
use config::{get_port, parse_config, css, Config};

/// Setup webserver then launch it
fn main() {
    let drain = slog_term::streamer().async().full().build();
    let drain = AtomicSwitch::new(drain);
    let root = Logger::root(drain.fuse(), o!());
    let log = root.new(o!());

    let args = App::new("static")
        .version("0.1.0")
        .author("Michael Gattozzi <mgattozzi@gmail.com>")
        .about("Personal website")
        .get_matches();

    if let Some(config) = parse_config(&log) {
        let port = get_port(&config);
        let address = format!("127.0.0.1:{}", port);

        Iron::new(setup(config, &address, root))
             .http(address.as_str())
             .expect("Failed to start website");

    } else {

        error!(log, "Failed to parse configuration. Make sure your Site.toml file is correct.");

    }
}

fn setup(conf: Config, address: &str, root: Logger) -> Mount {
    let log = root.new(o!());
    info!(log, "Setting up server");

    if !exists(&mkpath("site")) {
        // site doesn't exist yet so create it
        let _ = create_dir("site");
    }

    let mut mount = Mount::new();

    if let Some(c) = css(&conf) {
        compile_css(&c, &log);
    }

    mount_dirs(&mut mount, &log);

    file_updater(&conf, root);

    info!(log, "Server now running on {}", address);

    mount
}
