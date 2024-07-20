mod shape;
mod circle;
mod square;

use crate::shape::Shape;

fn main() {
    let obj1 = circle::Circle{ radius: 10.0 };
    println!("area of circle: {:.2}", obj1.area());

    let obj2 = square::Square{ side: 10 };
    println!("area of square: {:}", obj2.area());
}
