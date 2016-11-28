// Web Library Imports
use staticfile::Static;
use mount::Mount;

use std::fs::read_dir;
use util::mkpath;

/// Mount directories prior to launch
pub fn mount_dirs(mount: &mut Mount) {
    println!("Mounting files");

    // Mount all of the assets
    mount.mount("/css", Static::new(mkpath("assets/css/")));
    mount.mount("/js", Static::new(mkpath("assets/js/")));
    mount.mount("/images", Static::new(mkpath("assets/images/")));

    // Hardcode the starting page to the root
    mount.mount("/", Static::new(mkpath("site/")));

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
