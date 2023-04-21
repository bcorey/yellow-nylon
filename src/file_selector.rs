use native_dialog::FileDialog;

pub enum FileType {
    Database,
    Image
}

impl FileType {
    pub fn description(&self) -> &str {
        match self {
            FileType::Database => "SQLite3",
            FileType::Image => "Image",
        }
    }

    pub fn extensions(&self) -> &[&str] {
        match self {
            FileType::Database => &["db", ""],
            FileType::Image => &["jpg", "png"],
        }
    }
}

pub fn choose_file(file_type: FileType) -> Option<String> {
    let path = FileDialog::new()
        .set_location(r#"C:\Users\benja\Documents"#)
        .add_filter(file_type.description(), file_type.extensions())
        .show_open_single_file()
        .unwrap();
    
    match path {
        Some(path) => Some(path.to_str().unwrap().to_string()),
        None => None,
    }
}

pub fn choose_path() -> String {
    let path = FileDialog::new()
        .set_location(r#"C:\Users\benja\Documents"#)
        .show_open_single_dir()
        .unwrap();

    match path {
        Some(path) => path.to_str().unwrap().to_string(),
        None => "No path selected".to_string(),
    }
}

