mod calculation_spec;
mod two_dimensional;

use crate::calculation_spec::Area;

fn main() {
    // let circle_one = two_dimensional::Circle{ radius: 10 };
    let circle_one = new_circle(5);
    // println!("circle area: {}", circle_one.calculate());
    calculate_and_print_result("circle".to_string(), &circle_one);

    // let square_one = two_dimensional::Square{ length: 5 };
    let square_one = new_square(10);
    // println!("square area: {}", square_one.calculate());
    calculate_and_print_result("square".to_string(), &square_one);
}

fn new_circle(radius: i32) -> impl Area {
    let data = two_dimensional::Circle{
        radius
    };
    data
}

fn new_square(length: i32) -> impl Area {
    two_dimensional::Square{
        length
    }
}

fn calculate_and_print_result(name: String, item: &impl Area) {
    println!("{} area: {}", name, item.calculate());
}

fn calculate_and_print_result2<T: Area>(name: String, item: &T) {
    println!("{} area: {}", name, item.calculate());
}

fn calculate_and_print_result3<T>(name: String, item: &T) where T: Area {
    println!("{} area: {}", name, item.calculate());
}

fn calculate_and_print_result4<T>(name: String, item: &T)
where
    T: Area,
{
    println!("{} area: {}", name, item.calculate());
}