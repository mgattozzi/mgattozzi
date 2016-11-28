// Hashing Imports
use crypto::md5::Md5;
use crypto::digest::Digest;

use std::path::Path;
use std::fs::{read_dir, File};
use std::io::Read;
use std::time::Duration;
use std::thread::{spawn, sleep};

use config::{css, Config, PreProc};
use util::mkpath;
use md::render_pages;
use css::compile_css;

pub fn file_updater(conf: &Config) {
    if let Some(c) = css(conf) {
        let _ = spawn(move || {
            let dir = mkpath("sass");
            watch_css(dir, &c);
        });
    }

    let _ = spawn(move || {
        let dir = mkpath("pages");
        watch_pages(dir);
    });

    let _ = spawn(move || {
        let dir = mkpath("includes");
        watch_pages(dir);
    });
}

fn watch_pages(dir: &Path){
    // Create our duration object so the thread knows how long
    // to sleep
    let duration = Duration::new(5, 0);

    let mut md5_buf = String::new();
    let mut md5_comp = String::new();

    // Get the hash of all files as is
    md5sum(&mut md5_comp, &dir);


    loop {
        // Sum our directory then compare replace the current comparison
        // if it's different run the function passed in to be run on difference
        md5sum(&mut md5_buf, &dir);
        if md5_buf != md5_comp {
            md5_comp = md5_buf.clone();
            render_pages();
        }
        sleep(duration);
        // Clear our buffer for another pass
        md5_buf.clear();
    }
}

// TODO: Find a way to dedupe this code somehow
fn watch_css(dir: &Path, pre: &PreProc){
    // Create our duration object so the thread knows how long
    // to sleep
    let duration = Duration::new(5, 0);

    let mut md5_buf = String::new();
    let mut md5_comp = String::new();

    // Get the hash of all files as is
    md5sum(&mut md5_comp, &dir);


    loop {
        // Sum our directory then compare replace the current comparison
        // if it's different run the function passed in to be run on difference
        md5sum(&mut md5_buf, &dir);
        if md5_buf != md5_comp {
            md5_comp = md5_buf.clone();
            compile_css(&pre);
        }
        sleep(duration);
        // Clear our buffer for another pass
        md5_buf.clear();
    }
}

fn md5sum(buffer: &mut String, path: &Path) {
    // Get a file's contents, md5 it then push into buffer
    // Do so for all files in the directory recursively
    match read_dir(path) {
        Ok(iter) => {
            for entry in iter {
                match entry {
                    Ok(dir) => {
                        let name = dir.path();
                        if name.is_dir() {
                            md5sum(buffer, &name);
                        } else {
                            let mut md5 = Md5::new();
                            let mut f = File::open(name)
                                .expect("Unable to open file for md5sum");
                            let mut buff = String::new();
                            let _ = f.read_to_string(&mut buff);
                            md5.input_str(&buff);
                            buffer.push_str(&md5.result_str());
                        }
                    },
                    Err(_) => panic!("Unable to sum files for site"),
                }
            }
        },
        Err(_) => panic!("Code not run from project root"),
    }
}
