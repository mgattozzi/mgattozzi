extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate spongedown;

// Markdown Imports
use spongedown::parse;

// Web Library Imports
use iron::Iron;
use staticfile::Static;
use mount::Mount;

// Standard Library Imports
use std::path::{Path, PathBuf};
use std::fs::{read_dir, File};
use std::io::{Read, Write};

/// Setup webserver then launch it
fn main() {
    let mut mount = Mount::new();
    render_posts();
    mount_dirs(&mut mount);

    println!("Server now running");
    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}

/// Mount directories prior to launch
fn mount_dirs(mount: &mut Mount) {

    // Mount all of the assets
    mount.mount("/", Static::new(Path::new("assets/css/")));
    mount.mount("/", Static::new(Path::new("assets/js/")));

    // Hardcode the starting page to the root
    mount.mount("/", Static::new(Path::new("site/index.html")));

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

}

/// Takes all posts under _posts and renders them into html and places them
/// under site/posts to be served from
fn render_posts() {
    match read_dir("_posts") {
        Ok(iter) => {
            let mut posts = PathBuf::from("site/posts");
            for entry in iter {
                match entry {
                    Ok(dir) => {
                        let path = dir.path();
                        if path.is_file() && is_md_file(&path) {

                            // Setup path for output
                            posts.push(path.file_name().expect("Couldn't change filename for posts"));
                            posts.set_extension("html");

                            // Read in file to a string
                            let mut md = File::open(path).expect("Couldn't open md to render");
                            let mut buf = String::new();
                            md.read_to_string(&mut buf).expect("Couldn't read md file");

                            // Parse string then write it to the html file
                            let marked = parse(&buf);
                            let mut html = File::create(&posts).expect("Unable to create html file");
                            let _ = html.write_all(marked.as_bytes());

                            // Get it ready for next one
                            posts.pop();
                        }
                    },
                    Err(_) => panic!("Unable to mount directories for site"),
                }
            }
        },
        Err(_) => panic!("Code not run from project root"),
    }
}

fn is_md_file(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => ext == "md",
        None => false,
    }
}
