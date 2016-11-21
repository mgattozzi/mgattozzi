// Web Libraries
extern crate iron;
extern crate staticfile;
extern crate mount;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

// Standard Library Imports
use std::path::Path;
use std::fs::read_dir;

/// Setup webserver then launch it
fn main() {
    let mut mount = Mount::new();
    mount_dirs(&mut mount);

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
