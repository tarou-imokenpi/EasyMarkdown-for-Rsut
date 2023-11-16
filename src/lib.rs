use std::fs::{read_dir, File};
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;

pub struct MarkText {
    pub markdown_text: String,
}
pub trait Editor {
    fn show_text(&self);
    fn export_file(&self, file_name: &str);
    fn add_header(&mut self, level: u8, text: &str);
    fn add_code_block(&mut self, code_type: &str, text: &str);
    fn add_code_block_from_file(&mut self, code_type: &str, path: &PathBuf);
    fn add_inline_block(&mut self, text: &str);
}

impl Editor for MarkText {
    fn add_header(&mut self, level: u8, text: &str) {
        if level == 1 {
            self.markdown_text += &format!("# {}\n\n", text);
        } else if level == 2 {
            self.markdown_text += &format!("## {}\n\n", text);
        } else if level == 3 {
            self.markdown_text += &format!("### {}\n\n", text);
        } else if level == 4 {
            self.markdown_text += &format!("#### {}\n\n", text);
        } else if level == 5 {
            self.markdown_text += &format!("##### {}\n\n", text);
        } else if level == 6 {
            self.markdown_text += &format!("###### {}\n\n", text);
        } else {
            panic!(
                "^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n This level({}) is not available.\n^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        Can use 1,2,3,4,5,6\n_________________________________\n",
                level,
            );
        }
    }
    fn add_code_block_from_file(&mut self, code_type: &str, path: &PathBuf) {
        let mut f: File = File::open(path).expect("file not found");

        let mut contents: String = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        self.markdown_text += &format!("```{}\n{}\n```\n\n", code_type, &contents);
    }

    fn add_code_block(&mut self, code_type: &str, text: &str) {
        self.markdown_text += &format!("```{}\n{}\n```\n\n", code_type, text);
    }
    fn show_text(&self) {
        println!("{}", self.markdown_text);
    }

    fn add_inline_block(&mut self, _text: &str) {
        todo!()
    }
    fn export_file(&self, file_name: &str) {
        let output_file_name = file_name;
        let f: File = File::create(output_file_name).expect("Failed create file");
        let mut bfw: BufWriter<File> = BufWriter::new(f);
        match bfw.write_all(self.markdown_text.as_bytes()) {
            Err(e) => format_err(&format!("Failed to write to buffer: {}", e)),
            Ok(_) => println!("success!! \n{}", self.markdown_text),
        }
    }
}

fn format_err(message: &str) {
    eprintln!(
        "-----------------------------------------\n{}\n-----------------------------------------",
        message
    );
}
