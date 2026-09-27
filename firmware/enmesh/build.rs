
fn main() {
    println!("Generating bitmaps for assets...");

    let out_dir = std::env::var_os("OUT_DIR").unwrap_or("generated".into());
    let dest_dir = std::path::Path::new(&out_dir);

    // read the entries in the assets folder
    for result in std::fs::read_dir("assets").expect("ERROR: assets folder must exist at root of project")
    {
        if let Ok(entry) = result {
            // doesn't support subdirectories
            if entry.path().is_file() {
                // the file name extension to determine how to handle the file
                if let Some(extension) = entry.path().extension()
                {
                    use crate::asset_generator::*;
                    match extension.to_ascii_uppercase().to_str().expect("ERROR: file name is not ascii") {
                        "OTF" => ttf_generate(&entry.path(), dest_dir),
                        "TTF" => ttf_generate(&entry.path(), dest_dir),
                        "SVG" => svg_generate(&entry.path(), dest_dir),
                        _ => {
                            println!("cargo::warning=Unrecognized asset {:?}", entry.file_name());
                        }
                    }
                }
            }
        }
    }
}

mod asset_generator {

    pub fn svg_generate(svg_file: &std::path::Path, dest_dir: &std::path::Path)
    {
        // TODO
    }

    pub fn ttf_generate(svg_file: &std::path::Path, dest_dir: &std::path::Path)
    {

    }
}