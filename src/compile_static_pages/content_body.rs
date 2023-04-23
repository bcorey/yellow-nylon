use std::fs;
use std::io::Write;
use super::path_manager::PathManager;

pub fn build_html(md: String, path_manager: PathManager) {
    let html = markdown::to_html(&md);
    let html = add_server_prefix(html, path_manager.content_dir_server());
    write_to_file(html, path_manager.html_path());
}


fn write_to_file(html: String, html_path: String) {
    let mut new_file = fs::File::create(&html_path)
        .expect("Could not create html file");
    new_file.write_all(&html.as_bytes())
        .expect("Could not write data to html file");
}

fn add_server_prefix(html_string: String, server_prefix: String) -> String {
    let mut html_bits: Vec<String> = Vec::new();
    html_bits = recursion(html_string, html_bits, server_prefix);
    let mut html_final = String::new();
    for bit in html_bits {
        html_final.push_str(&bit);
    }

    html_final
}

fn recursion(html_string: String, mut html_bits: Vec<String>, path_prefix: String) -> Vec<String> {
    match html_string.find("src=") {
        Some(index) => {
            let (one, two) = html_string.split_at(index + 5usize);
            let mut one = one.to_string();
            one.push_str(&path_prefix);
            html_bits.push(one);
            html_bits = recursion(two.to_string(), html_bits, path_prefix);
        }
        None => html_bits.push(html_string),
    };

    html_bits
}