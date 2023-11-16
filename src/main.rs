use std::path::PathBuf;

use easy_markdown::*;

fn main() {
    let mut mark_text: MarkText = MarkText {
        markdown_text: String::new(),
    };

    mark_text.add_header(1, "heder text");
    mark_text.add_code_block("rust", "println!('hello');");

    let file_path = PathBuf::from("./lib.rs");
    mark_text.add_code_block_from_file("rust", &file_path);

    mark_text.export_file("output.md");
}
