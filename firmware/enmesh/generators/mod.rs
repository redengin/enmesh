pub fn ttf_generate(_ttf_file: &std::path::Path, _dir: &std::path::Path) {
    todo!();
    // println!("\tGenerating font assets for {:?}", ttf_file);
}

pub fn svg_generate(svg_file: &std::path::Path, dir: &std::path::Path) {
    println!("\tGenerating image assets for {:?}", svg_file);

    let svg_data = std::fs::read(svg_file).unwrap();
    let usvg_opt = resvg::usvg::Options::default();
    let svg_tree = resvg::usvg::Tree::from_data(&svg_data, &usvg_opt).unwrap();

    // create the toi files
    for height in [26, 36] {
        // render svg into sized pixmap (scaled to height)
        let scaling = (height as f32) / svg_tree.size().height();
        let width = (svg_tree.size().width() * scaling) as u32;
        let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height).unwrap();
        resvg::render(
            &svg_tree,
            resvg::usvg::Transform::default(),
            &mut pixmap.as_mut(),
        );

        // encode pixmpa to QOI
        let qoi_data = qoi::encode_to_vec(pixmap.data(), width, height).unwrap();

        // ensure the output dir exists
        std::fs::create_dir_all(dir).unwrap();

        println!("\t\tGenerating font assets for {:?}", svg_file);
        // create the QOI file
        let basename = svg_file
            .file_stem()
            .expect("files should always have a stem")
            .to_str()
            .expect("file names should be convertible to str");
        let qoi_filename = format!("{basename}_{width}_{height}.qoi");
        let qoi_path = dir.join(qoi_filename);
        let mut qoi_file = std::fs::File::create(qoi_path).unwrap();
        use std::io::Write;
        qoi_file.write_all(&qoi_data).unwrap();
    }
}
