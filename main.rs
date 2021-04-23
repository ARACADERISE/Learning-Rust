#[allow(unused_imports)]
use std::mem;

pub trait Info
{
    fn of_age(&self) -> bool;
}

struct MyInfo
{
    c_c: i32,
    age: u8,
    is_of_age: bool,
}

impl Info for MyInfo
{
    fn of_age(&self) -> bool
    {
        self.is_of_age
    }
}

impl MyInfo
{
    pub fn new(age: u8) -> Self
    {
        Self {
            c_c: 0,
            age: age,
            is_of_age: if age >= 21 { true } else { false }
        }
    }
    pub fn age_from_legal_age(&mut self) -> u8 
    { 
        self.c_c += 1;
        21 - self.age 
    }
}

fn is_of_age(var: &dyn Info)
{
    if var.of_age() == true
    {
        println!("IS OF AGE");
    } else
    {
        println!("NOT OF AGE :c");
    }
}

fn main()
{
    let mut info: MyInfo = MyInfo::new(16);
    
    println!("Age: {}, Is Of Age: {}", info.age, info.is_of_age);
    
    if info.is_of_age == false
    {
        is_of_age(&info);
        println!("You have {} more years until you're legal!", info.age_from_legal_age());
    }
    
    println!("We called {} function(s)!", info.c_c);
}
