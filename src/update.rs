// Hashing Imports
use seahash::hash;
use rayon::prelude::*;

use std::path::Path;
use std::fs::{read_dir, File};
use std::io::Read;
use std::time::Duration;
use std::thread::{spawn, sleep};

use config::{css, update_duration, Config, PreProc};
use util::mkpath;
use md::render_pages;
use css::compile_css;

pub fn file_updater(conf: &Config) {
    let duration = update_duration(conf);

    if let Some(c) = css(conf) {
        let _ = spawn(move || {
            let dir = mkpath("sass");
            watch_css(duration, dir, &c);
        });
    }

    let _ = spawn(move || {
        let dir = mkpath("pages");
        watch_pages(duration, dir);
    });

    let _ = spawn(move || {
        let dir = mkpath("includes");
        watch_pages(duration, dir);
    });
}

fn watch_pages(dur: u64, dir: &Path){
    // Create our duration object so the thread knows how long
    // to sleep
    let duration = Duration::new(dur, 0);

    let mut sea_buf: String;
    let mut sea_comp: String;

    // Get the hash of all files as is
    sea_comp = seahash(&dir);


    loop {
        // Sum our directory then compare replace the current comparison
        // if it's different run the function passed in to be run on difference
        sea_buf = seahash(&dir);
        if sea_buf != sea_comp {
            sea_comp = sea_buf.clone();
            render_pages();
        }
        sleep(duration);
        // Clear our buffer for another pass
        sea_buf.clear();
    }
}

// TODO: Find a way to dedupe this code somehow
fn watch_css(dur: u64, dir: &Path, pre: &PreProc){
    // Create our duration object so the thread knows how long
    // to sleep
    let duration = Duration::new(dur, 0);

    let mut sea_buf: String;
    let mut sea_comp: String;

    // Get the hash of all files as is
    sea_comp = seahash(&dir);


    loop {
        // Sum our directory then compare replace the current comparison
        // if it's different run the function passed in to be run on difference
        sea_buf = seahash(&dir);
        if sea_buf != sea_comp {
            sea_comp = sea_buf.clone();
            compile_css(&pre);
        }
        sleep(duration);
        // Clear our buffer for another pass
        sea_buf.clear();
    }
}

fn seahash(path: &Path) -> String {
    // Get a file's contents, sea it then push into buffer
    // Do so for all files in the directory recursively
    let mut hashed:Vec<u64> = Vec::new();

    match read_dir(path) {
        Ok(iter) => {
            for entry in iter {
                match entry {
                    Ok(dir) => {
                        let name = dir.path();
                        if name.is_dir() {
                            seahash(&name);
                        } else {
                            let mut f = File::open(name)
                                .expect("Unable to open file for seahash");
                            let mut buff = Vec::new();
                            let _ = f.read_to_end(&mut buff);
                            hashed.push(hash(&buff.as_slice()));
                        }
                    },
                    Err(_) => panic!("Unable to sum files for site"),
                }
            }
        },
        Err(_) => panic!("Code not run from project root"),
    }

    hashed.into_par_iter()
          .map(|h| h.to_string())
          .reduce(|| "".to_string(), |mut acc, h|
                {
                    acc.push_str(&h);
                    acc
                })

}
