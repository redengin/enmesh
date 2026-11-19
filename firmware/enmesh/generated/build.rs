//! This script generates the fonts and images modules from the assets folder.
//! Cargo runs this script before it compiles this crate.
//! The script writes all output to OUT_DIR.

#[path = "../generators/mod.rs"]
mod generators;

/// provide write trait for File I/O
use std::io::Write;

/// folder for assets
const ASSETS_PATH: &str = "../assets";

fn main() {
    println!("cargo::rerun-if-changed={ASSETS_PATH}");
    println!("cargo::rerun-if-changed=../generators");

    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    // begin the fonts module
    let mut fonts_rs = std::fs::File::create(out_dir.join("fonts.rs")).unwrap();
    fonts_rs
        .write_all(b"// build.rs generates this file. Do not edit it.\n\n")
        .unwrap();

    // begin the images module
    let mut images_rs = std::fs::File::create(out_dir.join("images.rs")).unwrap();
    images_rs
        .write_all(
            b"\
            // build.rs generates this file. Do not edit it.\n\
            \n\
            /// provide the common crates via re-export\n\
            use common::*;\n\
            ",
        )
        .unwrap();

    // read the entries in the assets folder
    for result in
        std::fs::read_dir(ASSETS_PATH).expect("ERROR: assets folder must exist at root of project")
    {
        if let Ok(entry) = result {
            // ignores subdirectories
            if entry.path().is_file() {
                // the file name extension to determine how to handle the file
                if let Some(extension) = entry.path().extension() {
                    /// provide access to transformations
                    use generators::*;
                    match extension
                        .to_ascii_uppercase()
                        .to_str()
                        .expect("ERROR: file name is not ascii")
                    {
                        "OTF" => ttf_generate(&entry.path(), &out_dir, &mut fonts_rs),
                        "TTF" => ttf_generate(&entry.path(), &out_dir, &mut fonts_rs),
                        "SVG" => svg_generate(&entry.path(), &out_dir, &mut images_rs),
                        _ => {
                            println!("cargo::warning=Unrecognized asset {:?}", entry.file_name());
                        }
                    }
                }
            }
        }
    }
}
