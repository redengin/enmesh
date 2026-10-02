mod generators;

fn main() {
    println!("Generating bitmaps for assets...");

    /// folder for assets
    const ASSETS_PATH: &str = "assets";
    /// folder for generated content
    const GENERARTED_PATH: &str = "generated";
    let dest_dir = std::path::Path::new(GENERARTED_PATH);

    // subfolder for fonts
    let fonts_dir = dest_dir.join("fonts");
    // subfolder for images
    let images_dir = dest_dir.join("images");

    // read the entries in the assets folder
    for result in std::fs::read_dir(ASSETS_PATH).expect("ERROR: assets folder must exist at root of project")
    {
        if let Ok(entry) = result {
            // ignores subdirectories
            if entry.path().is_file() {
                // the file name extension to determine how to handle the file
                if let Some(extension) = entry.path().extension()
                {
                    /// provide access to transformations
                    use generators::*;
                    match extension.to_ascii_uppercase().to_str().expect("ERROR: file name is not ascii") {
                        "OTF" => ttf_generate(&entry.path(), &fonts_dir),
                        "TTF" => ttf_generate(&entry.path(), &fonts_dir),
                        "SVG" => svg_generate(&entry.path(), &images_dir),
                        _ => {
                            println!("cargo::warning=Unrecognized asset {:?}", entry.file_name());
                        }
                    }
                }
            }
        }
    }
}

