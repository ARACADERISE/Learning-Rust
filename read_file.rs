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

#[derive(Debug)]
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
            info.file_info.push(line?);
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
}

fn main()
{
    //IoErro::panic_if_format_error("?");

    let val = Vals::new(&PathBuf::from(".due"));
    println!("{:?}", val);
}
