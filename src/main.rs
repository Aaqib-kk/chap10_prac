use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
struct Numbers <T>
{
    x: T,
    y: T,
}

impl <T> Add for Numbers <T>
where T: Add<Output = T> 
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output
    {
        Self
        {
            // self.x + self.y
            x: self.x + other.x,
            y: self.y + other.y,
        }

    }
}

fn main() 
{
    let number_list = Numbers
    {
        x: 5,
        y: 10,
    };
    println!("{:?}", number_list.x + number_list.y);
}










// use std::ops::{Add, Sub};

// #[derive(Debug, PartialEq)]
// pub struct Point {
//     x: i32,
//     y: i32,
// }

// impl Add for Point {
//     type Output = Point;

//     fn add(self, other: Point) -> Point {
//         Point {x: self.x + other.x, y: self.y + other.y}
//     }
// }

// impl Sub for Point {
//     type Output = Point;

//     fn sub(self, other: Point) -> Point {
//         Point {x: self.x - other.x, y: self.y - other.y}
//     }
// }
// fn main()
// {
//     let point1 = {Point {x: 1, y: 0} + Point {x: 2, y: 3} };
//     println!("{:?}",point1);
// }
