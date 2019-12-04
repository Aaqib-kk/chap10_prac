// use std::fmt::Display;
pub struct Numbers <T>
{
    x: T,
    y: T,
}

pub trait Addition <T>
{
    fn add(self) -> T;
}

impl <T> Addition <T> for Numbers <T> 
{
    fn add(self) -> T
    {
       self.x + self.y 
    }
}

fn main() 
{
    let number_list = Numbers
    {
        x: 5,
        y: 10,
    };

    println!("{}", number_list.add());
}
