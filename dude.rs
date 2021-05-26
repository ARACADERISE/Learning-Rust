#![allow(unused_imports)]
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct FileInfo {
    dir: PathBuf,
    content: String,
    is_open: bool,
    is_path: bool
}

#[derive(Debug)]
enum Errors {
    IoError(io::Error),
    NoDir(PathBuf),
    NoFile(PathBuf)
}

#[derive(Debug)]
enum Success {
    DirExists(FileInfo),
    FileExists(FileInfo)
}

#[allow(dead_code)]
impl Success {
    fn dir_exists(fi: FileInfo) -> Success {
        Success::DirExists(fi)
    }
    fn file_exists(fi: FileInfo) -> Success {
        Success::FileExists(fi)
    }
}

#[allow(dead_code)]
impl Errors {
    fn no_dir(dirname: PathBuf) -> Errors {
        Errors::NoDir(dirname)
    }
    fn no_file(filename: PathBuf) -> Errors {
        Errors::NoFile(filename)
    }
}

impl From<io::Error> for Errors {
    fn from(err: io::Error) -> Errors {
        Errors::IoError(err)
    }
}

#[allow(dead_code)] // some functions we won't use
impl FileInfo {
    fn new() -> Self {
        Self {
            dir: PathBuf::new(),
            content: String::new(),
            is_open: false,
            is_path: true
        }
    }
    
    fn open_path(&mut self, path: PathBuf, file: String) -> Result<Success, Errors> {
        self.dir = path.join(file);
        
        if self.check_path() == true {
            return Ok(Success::dir_exists(self.clone()));
        }
        
        Err(Errors::no_dir(self.dir.to_path_buf()))
    }
    
    fn check_path(&mut self) -> bool {
        
        if self.dir.exists() {
            self.is_path = true;
            return true;
        }
        
        return false;
    }
    
    fn read_file(&mut self) -> Result<(), io::Error> {
        if self.is_path == true {
            self.is_open = true;
            let content = BufReader::new(File::open(&self.dir)?);
            
            let mut info = String::new();
            
            for line in content.lines() {
                info.push_str(&line?);
            }
            
            self.content = info;
        
            return Ok(());
        }
        Ok(())
    }
    
    fn read(&mut self) {
        match self.open_path(PathBuf::from("/"), "playground/src/main.rs".to_string()) {
            Ok(t) => {
                match t {
                    Success::DirExists(_) => {
                        match self.read_file() {
                            Ok(()) => println!("{:?}", self),
                            Err(t) => panic!("{:?}", t)
                        }
                    },
                    _ => {}
                }
            },
            Err(t) => panic!("{:?}", t)
        }
    }
}

fn main() {
    let mut info = FileInfo::new();
    
    info.read();
}
