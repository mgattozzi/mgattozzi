use std::process::Command;
use std::io::Write;
use std::fs::File;
use util::mkpath;
use md::render_pages;

pub fn compile_sass() {
    println!("Compiling sass");

    let output = Command::new("sass")
                        .arg("sass/styles.scss")
                        .output()
                        .expect("sass compilation failed")
                        .stdout;

    let sass = String::from_utf8_lossy(&output);

    let mut css = File::create(mkpath("assets/css/main.css"))
        .expect("Unable to create css file");
    let _ = css.write_all(sass.as_bytes());

    println!("Compiling sass completed");
    render_pages();
}
