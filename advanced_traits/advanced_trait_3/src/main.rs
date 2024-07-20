mod calculation_spec;
mod two_dimensional;

use crate::calculation_spec::Area;
use crate::calculation_spec::Circumference;

fn main() {
    let circle_one = new_circle(5);
    calculate_and_print_result6("circle".to_string(), &circle_one);

    let square_one = new_square(10);
    calculate_and_print_result7("square".to_string(), &square_one);
}

fn new_circle(radius: i32) -> impl Area {
    let data = two_dimensional::Circle{
        radius
    };
    data
}

fn new_square(length: i32) -> impl Area + Circumference {
    two_dimensional::Square{
        length
    }
}

fn calculate_and_print_result6<T>(name: String, item: &T)
where
    T: Area,
{
    println!("{} area: {}", name, item.calculate_area());
}

fn calculate_and_print_result7<T>(name: String, item: &T)
where
    T: Area + Circumference,
{
    println!("{} area: {}", name, item.calculate_area());
    println!("{} circumference: {}", name, item.calculate_circumference());
}
