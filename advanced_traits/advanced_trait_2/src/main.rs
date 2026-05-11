mod calculation_spec;
mod two_dimensional;

use crate::calculation_spec::Area;
use crate::calculation_spec::Circumference;

fn main() {
    // A.37.3 - Parameter bertipe lebih dari 1 trait
    let circle_one = two_dimensional::Circle{ radius: 10 };
    calculate_and_print_result("circle".to_string(), &circle_one);

    let square_one = two_dimensional::Square{ length: 5 };
    calculate_and_print_result("square".to_string(), &square_one);
}

// A.37.3 - Trait sebagai tipe parameter dengan multi trait
fn calculate_and_print_result(name: String, item: &(impl Area + Circumference)) {
    println!("{} area: {}", name, item.calculate_area());
    println!("{} circumference: {}", name, item.calculate_circumference());
}

// A.37.4 - Trait bound syntax (single trait)
fn calculate_and_print_result2<T: Area>(name: String, item: &T) {
    println!("{} area: {}", name, item.calculate_area());
}

// A.37.4 - Trait bound syntax (multi trait)
fn calculate_and_print_result3<T: Area + Circumference>(name: String, item: &T) {
    println!("{} area: {}", name, item.calculate_area());
    println!("{} circumference: {}", name, item.calculate_circumference());
}

// A.37.5 - Trait `where` clause (inline)
fn calculate_and_print_result4<T>(name: String, item: &T) where T: Area + Circumference {
    println!("{} area: {}", name, item.calculate_area());
}

// A.37.5 - Trait `where` clause (multi-line)
fn calculate_and_print_result5<T>(name: String, item: &T)
where
    T: Area + Circumference,
    // ... other generic params if exists
{
    println!("{} area: {}", name, item.calculate_area());
}