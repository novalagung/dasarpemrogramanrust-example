mod calculation_spec;
mod two_dimensional;

use crate::calculation_spec::Area;

fn main() {
    // let circle_one = two_dimensional::Circle{ radius: 10 };
    // println!("circle area: {}", circle_one.calculate_area());

    // let square_one = two_dimensional::Square{ length: 5 };
    // println!("square area: {}", square_one.calculate_area());

    let circle_one = two_dimensional::Circle{ radius: 10 };
    calculate_and_print_result("circle".to_string(), &circle_one);

    let square_one = two_dimensional::Square{ length: 5 };
    calculate_and_print_result("square".to_string(), &square_one);
}

fn calculate_and_print_result(name: String, item: &impl Area) {
    println!("{} area: {}", name, item.calculate_area());
}
