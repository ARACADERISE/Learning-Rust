#![allow(unused_imports)]
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Info {
    path: PathBuf,
    valid_file: Result<PathBuf, String>,
    file_info: Option<&'static str>
}

trait Functions {
    fn new() -> Self; // return empty struct
    fn assign(&mut self, dir: PathBuf, file: String) -> &Self;
}

impl Functions for Info {
    fn new() -> Self {
        Self {path: PathBuf::from("."), valid_file: Ok(PathBuf::from(".")), file_info: None }
    }
    fn assign(&mut self, dir: PathBuf, file: String) -> &Self {
        self.path = dir.join(file);
        
        if !self.path.exists() {
            self.valid_file = Err(format!("The file {:?} does not exist", self.path));
            self.file_info = None;
            return self
        }
        
        self.valid_file = Ok(self.path.to_path_buf());
        self.file_info = None;
        return self;
    }
}

fn main() {
    let mut info = Info::new();
    info.assign(PathBuf::from("."), "main.rss".to_string());
    
    println!("{:?}", info);
}
