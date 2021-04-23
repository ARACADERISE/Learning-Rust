pub mod info
{
    pub trait Info
    {
        fn new(name: &'static str, age: u8) -> Self;
        fn is_of_age(&self) -> bool;
        fn print_all(&self);
    }

    #[derive(Debug)]
    pub struct MyInfo
    {
        pub name: &'static str,
        pub age: u8,
    }

    impl Info for MyInfo
    {
        fn new(name: &'static str, age: u8) -> Self { Self { name: name, age: age } }
        fn is_of_age(&self) -> bool
        {
            if self.age >= 21
            {
                return true;
            }
            return false
        }
        fn print_all(&self)
        {
            println!("{:?}", self);
        }
    }
}

use info::Info;

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main()
{
    let _info = info::MyInfo::new("Aidan", 16);
    _info.print_all();
    println!("Hello, World!");
}
