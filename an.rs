#![allow(unused_imports)]
use std::path::PathBuf;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead, Error, ErrorKind};

#[derive(Debug)]
enum IoErrors {
    IOError(io::Error),
    NoSuchFile,
    EmptyFile,
    InvalidChar
}

#[derive(Debug)]
struct Info {
    path: PathBuf,
    valid_file: Result<PathBuf, IoErrors>,
    file_info: Option<Vec<String>>
}

impl From<io::Error> for IoErrors {
    fn from(err: io::Error) -> IoErrors {
        return IoErrors::IOError(err);
    }
}

impl IoErrors {
    fn io_err(err: Error) -> IoErrors {
        return IoErrors::IOError(err);
    } 
    fn empty_file() -> IoErrors {
        return IoErrors::EmptyFile;
    }
    fn invalid_char() -> IoErrors {
        return IoErrors::InvalidChar;
    }
    fn no_such_file() -> IoErrors {
        return IoErrors::NoSuchFile;
    }
}

trait Functions {
    fn new() -> Self; // return empty struct
    fn assign(&mut self, dir: PathBuf, file: String) -> Result<&Self, IoErrors>;
}

impl Functions for Info {
    fn new() -> Self {
        Self {path: PathBuf::from("."), valid_file: Ok(PathBuf::from(".")), file_info: None }
    }
    fn assign(&mut self, dir: PathBuf, file: String) -> Result<&Self, IoErrors> {
        self.path = dir.join(file);
        
        if !self.path.exists() {
            self.valid_file = Err(IoErrors::no_such_file());
            self.file_info = None;
            return Err(IoErrors::io_err(Error::new(ErrorKind::NotFound, "File Not Found")));
        }
        
        self.valid_file = Ok(self.path.to_path_buf());
        self.file_info = None;
        return Ok(self);
    }
}

#[allow(dead_code)]
impl Info {
    fn read_file(&mut self) -> Result<&Self, IoErrors> {
        match self.valid_file {
            Ok(ref t) => {
                let content = BufReader::new(File::open(t)?);
                
                let mut file_info: Vec<String> = Vec::new();
                for line in content.lines() {
                    file_info.push(line?);
                }
                if file_info.len() == 0 {
                    return Err(IoErrors::empty_file())
                }

                self.file_info = Some(file_info);
            },
            _ => {}
        }

        return Ok(self);
    }

    fn print_file_info(&mut self) {
        match self.read_file() {
            Ok(ref t) => {
                match &t.file_info {
                    Some(f) => {
                        for i in 0..f.len() {
                            println!("{}", f[i]);
                        }
                    },
                    _ => {}
                }
            },
            Err(t) => panic!("{:?}", t)
        }
    }

    fn check_char(_char: u8) -> Result<char, IoErrors> {
        match _char {
            _ if _char < 0x41 || _char > 0x7a => return Err(IoErrors::invalid_char()),
            _ => {}
        }
        return Ok(_char as char);
    }

    fn gather_all_invalid(&mut self) -> Vec<char> {
        let mut ar: Vec<char> = Vec::new();
        match self.read_file() {
            Ok(ref t) => {
                match &t.file_info {
                    Some(f) => {
                        for i in 0..f.len() {
                            for x in f[i].chars() {
                                match Info::check_char(x as u8) {
                                    Ok(_) => {},
                                    Err(_) => ar.push(x)
                                }
                            }
                        }
                    },
                    _ => {}
                }
            
            },
            Err(t) => panic!("{:?}", t)
        }
        return ar;
    }
}

fn main() {
    let mut info = Info::new();
    match info.assign(PathBuf::from("."), "main.rss".to_string()) {
        Err(ref t) => println!("{:?}", t),
        _ => {}
    }

    match info.valid_file {
        Ok(_) => {},
        Err(t) => panic!("{:?}", t)
    }
}
