use std::path::PathBuf;

#[derive(Debug, Clone)]
struct Info {
    path: PathBuf,
    all: Vec<Info>
}

#[derive(Debug)]
enum FileErrors {
    FileNotExists(String)
}

#[allow(unreachable_patterns)] // For derefernece_error
impl FileErrors {
    fn new_file_error(format: String) -> FileErrors { FileErrors::FileNotExists(format) }
    
    fn derefernece_error(&self) -> String {
        match self {
            FileErrors::FileNotExists(t) => return t.to_string(),
            _ => return self.derefernece_error() // should never get here
        }
    }
}

impl Info {
    fn new(path: PathBuf, value: String) -> Self {
        let mut info = Info {
            path: path.join(value),
            all: Vec::new()
        };
        
        info.all.push(info.clone());
        return info;
    }
    
    fn check_file(&self) -> Result<&Self, FileErrors> {
        if !self.path.exists() {
            return Err(FileErrors::new_file_error(format!("The file {:?} does not exist", self.path)));
        }
        return Ok(self);
    }
    
    fn dereference_all(&self) -> String {
        format!("ALL[{}] -> | PATH: {:?} \n\t  | ALL: {:?} \n\t  | SIZE: {} ", self.all.len() - 1, self.all[self.all.len()-1].path, self.all[self.all.len()-1].all, self.all[self.all.len()-1].all.len())
    }
    
    fn dereference_info(&self) -> String {
        format!("ORIGINAL -> | PATH: {:?} \n\t    | ALL: {:?} \n\t    | SIZE: {} \n\n{}", self.path, self.all, self.all.len(), self.dereference_all())
    }
}

fn main() {
    let info = Info::new(PathBuf::from("target/debug"), "playground".to_string());
    match info.check_file() {
        Ok(t) => println!("{}", t.dereference_info()),
        Err(t) => println!("{}", t.derefernece_error())
    }
    
}
