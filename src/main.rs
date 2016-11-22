extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate spongedown;
extern crate clap;
extern crate crypto;

// Command Argument Imports
use clap::{App, Arg};

// Hashing Imports
use crypto::md5::Md5;
use crypto::digest::Digest;

// Markdown Imports
use spongedown::parse;

// Web Library Imports
use iron::Iron;
use staticfile::Static;
use mount::Mount;

// Standard Library Imports
use std::path::{Path, PathBuf};
use std::fs::{create_dir,read_dir, metadata, File};
use std::io::{Read, Write};
use std::process::Command;
use std::thread::{spawn,sleep};
use std::time;

/// Setup webserver then launch it
fn main() {

    let args = App::new("mgattozzi")
        .version("0.1.0")
        .author("Michael Gattozzi <mgattozzi@gmail.com>")
        .about("Personal website")
        .arg(Arg::with_name("hotload")
            .short("h")
            .takes_value(true)
            .help("Hot reload files"))
        .get_matches();

    println!("Setting up server");

    if metadata("site").is_err() {
        // site doesn't exist yet so create it
        let _ = create_dir("site");
    }

    let mut mount = Mount::new();
    compile_sass();
    render_pages();
    mount_dirs(&mut mount);

    // We'll hotload after just to make sure everything is processed once
    if let Some(hot) = args.value_of("hotload") {
        let sleep_int = hot.parse::<u64>().expect("Use a number for input");

        let _ = spawn(move || {
            let dir = Path::new("_sass");
            watch_dir(sleep_int, dir, compile_sass);
        });

        let _ = spawn(move || {
            let dir = Path::new("_pages");
            watch_dir(sleep_int, dir, render_pages);
        });
    }

    println!("Server now running");
    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}

/// Mount directories prior to launch
fn mount_dirs(mount: &mut Mount) {
    println!("Mounting files");

    // Mount all of the assets
    mount.mount("/css", Static::new(Path::new("assets/css/")));
    mount.mount("/js", Static::new(Path::new("assets/js/")));
    mount.mount("/images", Static::new(Path::new("assets/images/")));

    // Hardcode the starting page to the root
    mount.mount("/", Static::new(Path::new("site/")));

    // Mount each directory to the site
    match read_dir("site") {
        Ok(iter) => {
            for entry in iter {
                match entry {
                    Ok(dir) => {
                        let path = dir.path();
                        if path.is_dir() {
                            // We can use expect here because we know there's a
                            // directory entry and if it fails to convert we'd
                            // want the program to crash anyways
                            mount.mount(dir.file_name()
                                           .to_str()
                                           .expect("Couldn't convert directory name and crashed")
                                        , Static::new(path));
                        }
                    },
                    Err(_) => panic!("Unable to mount directories for site"),
                }
            }
        },
        Err(_) => panic!("Code not run from project root"),
    }

    println!("Mounting files completed");
}

fn render_pages() {
    match read_dir("_pages") {
        Ok(iter) => {
            for entry in iter {
                match entry {
                    Ok(dir) => {
                        let mut to = PathBuf::from("site");
                        if dir.path().is_dir() {
                            to.push(dir.file_name()
                                    .to_str()
                                    .expect("Failure to get directory filename"));
                            render_md(&dir.path(), to);
                        }
                    },
                    Err(_) => panic!("Unable to read from directories for site"),
                }
            }
        },
        Err(_) => panic!("Code not run from project root"),
    }

    // Render the top level
    render_md(Path::new("_pages"), PathBuf::from("site"));
}

fn render_md(from: &Path, mut to: PathBuf) {
    println!("Rendering Markdown for {}", from.to_str().unwrap());
    match read_dir(from) {
        Ok(iter) => {

            if from != Path::new("_pages") && metadata(&to).is_err() {
                // Folder doesn't exist yet so create it
                let _ = create_dir(&to);
            }

            for entry in iter {
                match entry {
                    Ok(dir) => {
                        let path = dir.path();
                        if path.is_file() && is_md_file(&path) {

                            // Setup path for output
                            to.push(path.file_name().expect("Couldn't change filename for folder"));
                            to.set_extension("html");

                            // Read in file to a string
                            let mut md = File::open(path).expect("Couldn't open md to render");
                            let mut buf = String::new();
                            md.read_to_string(&mut buf).expect("Couldn't read md file");

                            // Parse string then write it to the html file
                            let mut marked = parse(&buf);

                            // Get the main style sheet automatically as part of the file
                            marked.push_str("<link rel=\"stylesheet\" type=\"text/css\" href=\"/css/main.css\">");
                            marked.push_str("<link rel=\"stylesheet\" href=\"/css/highlight/zenburn.css\">");
                            marked.push_str("<script src=\"/js/highlight/highlight.pack.js\"></script>");
                            marked.push_str("<script>hljs.initHighlightingOnLoad();</script>");

                            let mut html = File::create(&to).expect("Unable to create html file");
                            let _ = html.write_all(marked.as_bytes());

                            // Get it ready for next one
                            to.pop();
                        }
                    },
                    Err(_) => panic!("Unable to mount directories for site"),
                }
            }
        },
        Err(_) => panic!("Code not run from project root"),
    }
    println!("Rendering Markdown for {} completed", from.to_str().unwrap());
}

fn is_md_file(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => ext == "md",
        None => false,
    }
}

fn compile_sass() {
    println!("Compiling sass");

    let output = Command::new("sass")
                        .arg("_sass/styles.scss")
                        .output()
                        .expect("sass compilation failed")
                        .stdout;
    let sass = String::from_utf8_lossy(&output);

    let mut css = File::create(Path::new("assets/css/main.css"))
        .expect("Unable to create css file");
    let _ = css.write_all(sass.as_bytes());

    println!("Compiling sass completed");
}

fn watch_dir<F: Fn()>(interval: u64, dir: &Path, func: F){
    // Create our duration object so the thread knows how long
    // to sleep
    let duration = time::Duration::new(interval, 0);

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
            func();
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
