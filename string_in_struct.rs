pub trait Info
{
    fn new(name: &'static str, age: i8) -> Self;
}

#[derive(Debug)]
struct MyInfo
{
    name: &'static str,
    age: i8,
}

impl Info for MyInfo
{
    fn new(name: &'static str, age: i8) -> Self
    {
        Self {
            name: name,
            age: age,
        }
    }
}

fn main()
{
    let _info: MyInfo = MyInfo::new("Aidan", 16);
    
    println!("{:?}", _info);
}
