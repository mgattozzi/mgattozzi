// Markdown Imports
use spongedown::parse;

use util::*;
use std::path::{Path, PathBuf};
use std::fs::{create_dir,read_dir, File};
use std::io::{Read, Write};

pub fn render_pages() {
    match read_dir("pages") {
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
    render_md(mkpath("pages"), PathBuf::from("site"));
}

fn render_md(from: &Path, mut to: PathBuf) {
    println!("Rendering Markdown for {}", from.to_str().unwrap());
    let mut header = String::new();
    let mut head   = String::new();
    let mut footer = String::new();
    let mut post_foot: Option<String>;
    if !from.starts_with("pages/posts") {
        post_foot = None;
    } else {
        post_foot = Some(String::new());
    }

    read_in_includes(&mut header, &mut head, &mut footer, &mut post_foot);


    match read_dir(from) {
        Ok(iter) => {

            if from != mkpath("pages") && !exists(&to) {
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
                            let mut marked = String::new();
                            // This makes the page scale for mobile
                            marked.push_str(&head);
                            marked.push_str(&header);
                            marked.push_str(&parse(&buf));
                            if let Some(ref post) = post_foot {
                                marked.push_str(&post);
                            }
                            marked.push_str(&footer);


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

fn read_in_includes(mut h_buf: &mut String, mut b_buf: &mut String, mut f_buf: &mut String, mut p_buf: &mut Option<String>) {
    let h = mkpath("includes/header.html");
    let b = mkpath("includes/head.html");
    let c = mkpath("assets/css/main.css");
    let z = mkpath("assets/css/highlight/zenburn.css");
    let f = mkpath("includes/footer.html");

    let mut file_handle = File::open(h).expect("Couldn't open includes for site");
    file_handle.read_to_string(&mut h_buf).expect("Couldn't read include file");

    file_handle = File::open(b).expect("Couldn't open includes for site");
    file_handle.read_to_string(&mut b_buf).expect("Couldn't read include file");

    // Push the css inline
    file_handle = File::open(c).expect("Couldn't open includes for site");
    b_buf.push_str("<style>");
    file_handle.read_to_string(&mut b_buf).expect("Couldn't read include file");
    file_handle = File::open(z).expect("Couldn't open includes for site");
    file_handle.read_to_string(&mut b_buf).expect("Couldn't read include file");
    b_buf.push_str("</style>");

    file_handle = File::open(f).expect("couldn't open includes for site");
    file_handle.read_to_string(&mut f_buf).expect("couldn't read include file");

    if let &mut Some(ref mut post) = p_buf {
        let p = mkpath("includes/post.html");
        file_handle = File::open(p).expect("couldn't open includes for site");
        file_handle.read_to_string(post).expect("couldn't read include file");
    }
}

fn is_md_file(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => ext == "md",
        None => false,
    }
}
