#[derive(Debug)]
pub struct MyInfo
{
    name: &'static str,
    age: u8
}

pub trait Info
{
    fn new(name: &'static str, age: u8) -> Self;
    fn is_of_age(&self) -> Option<bool>;
    fn years_left(&self) -> u8;
}

impl Info for MyInfo
{
    fn new(name: &'static str, age: u8) -> Self
    {
        Self {
            name: name,
            age: age,
        }
    }
    fn is_of_age(&self) -> Option<bool>
    {
        match self.age
        {
            _ if self.age >= 21 => Some(true),
            _ => Some(false)
        }
    }
    fn years_left(&self) -> u8
    {
        21 - self.age
    }
}

#[allow(unused_mut)]
fn main()
{
    let mut info = MyInfo::new("Aidan", 16);

    println!("{:?} > Years Left: {}", info, match info.is_of_age() {
        Some(x) => {
            match x {
                true => "Already of age!".to_string(),
                false => info.years_left().to_string(),
            }
        },
        _ => panic!("ERROR OCCURRED"),
    });
}
