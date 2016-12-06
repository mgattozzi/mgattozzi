use std::process::Command;
use std::io::Write;
use std::fs::File;
use util::mkpath;
use md::render_pages;
use config::PreProc;
use config::PreProc::{Sass,Less};
use slog::Logger;

pub fn compile_css(proccesor: &PreProc, log: &Logger) {
    match proccesor {
        &Sass => compile_sass(log),
        &Less => compile_less(log),
    }
}

fn compile_sass(log: &Logger) {
    info!(log, "Compiling sass");
    let output = Command::new("sass")
                        .arg("sass/main.scss")
                        .output()
                        .expect("sass compilation failed")
                        .stdout;

    let sass = String::from_utf8_lossy(&output);

    let mut css = File::create(mkpath("assets/css/main.css"))
        .expect("Unable to create css file");
    let _ = css.write_all(sass.as_bytes());

    info!(log, "Compiling sass completed");
    render_pages(log);
}

fn compile_less(log: &Logger) {
    info!(log, "Compiling less");

    let output = Command::new("lessc")
                        .arg("less/main.less")
                        .output()
                        .expect("less compilation failed")
                        .stdout;

    let less = String::from_utf8_lossy(&output);

    let mut css = File::create(mkpath("assets/css/main.css"))
        .expect("Unable to create css file");
    let _ = css.write_all(less.as_bytes());

    info!(log, "Compiling less completed");
    render_pages(log);
}
