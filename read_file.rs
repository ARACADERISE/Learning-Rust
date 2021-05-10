#![allow(unused_imports)]
use std::string::String;
use std::path::Path;
use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::collections::BTreeMap;

enum IoErro
{
    InvalidFormat(String)
}

#[derive(Debug, Clone)]
pub struct Vals {
    path: PathBuf,
    file_info: Vec<String>,
}

#[allow(dead_code)]
impl IoErro
{
    fn check_suspected_format(format: &'static str) -> Result<&'static str, IoErro>
    {
        for i in format.as_bytes()
        {
            match i {
                _ if i < &0x41 || i > &0x7A => return Err(IoErro::InvalidFormat(format!("Error with format: {}.\n\tError at: {}", format, *i as char))),
                _ => {}
            }
        }
        Ok(format)
    }

    fn check_format(format: &'static str) -> Result<&'static str, String>
    {
        match IoErro::check_suspected_format(format) {
            Ok(t) => Ok(t),
            Err(IoErro::InvalidFormat(t)) => Err(t)
        }
    }

    fn panic_if_format_error(format: &'static str)
    {
        match IoErro::check_format(format)
        {
            Ok(_) => {},
            Err(t) => panic!{t}
        }
    }

    fn panic_if_invalid_char(val: u8) -> Result<char, String>
    {
        match val {
            _ if val < 0x41 || val > 0x7A => return Err(format!("{} is invalid", val as char)),
            _ => {}
        }
        Ok(val as char)
    }

    fn check_each_index(arr: Vec<String>) -> Result<Option<Vec<String>>, String>
    {
        for i in 0..arr.len() {
            for x in arr[i].chars() {
                match IoErro::panic_if_invalid_char(x as u8) {
                    Ok(_) => {},
                    Err(t) => return Err(t)
                }
            }
        }

        Ok(Some(arr))
    }
} 

impl Vals {
    fn new(path: &PathBuf) -> io::Result<Vals>
    {
        let mut info = Vals {
            path: path.join("HEAD"),
            file_info: Vec::new()
        };

        if !info.path.exists()
        {
            return Ok(info);
        }

        let content = BufReader::new(File::open(&info.path)?);
        for line in content.lines() {
            match line {
                Ok(t) => info.file_info.push(t),
                Err(t) => panic!{"Err: {}", t}
            }
        }

        let mut vec: Vec<String> = Vec::new();
        for i in 0..info.file_info.len() {
            let mut val: String = String::new();
            for x in info.file_info[i].chars() {
                if x == 0x20 as char {
                    vec.push(val.clone());
                    val = "".to_string();
                    continue;
                }
                if x == info.file_info[i].chars().last().unwrap() {
                    val.push(info.file_info[i].chars().last().unwrap());
                    vec.push(val.clone());
                    break;
                }
                val.push(x);
            }
        }
        info.file_info.clear();

        for i in vec {
            info.file_info.push(i);
        }

        Ok(info)
    }

    fn _loop(&self)
    {
        for i in 0..self.file_info.len()
        {
            println!("{}", self.file_info[i]);
        }
    }
}

fn main()
{
    //IoErro::panic_if_format_error("?");

    let val = Vals::new(&PathBuf::from(".due")).unwrap();
    match IoErro::check_each_index(val.file_info) {
        Ok(Some(t)) => println!("{:?}", t),
        Err(t) => println!("{}", t),
        _ => {} // shoud never get here
    }
}
