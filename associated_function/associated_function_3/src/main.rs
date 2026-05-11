// A.24.5 - tuple struct associated function
mod model;

fn main() {
    let red = model::Color::red();
    let green = model::Color::green();
    let blue = model::Color::blue();

    println!("{:#?} {:#?} {:#?}", red, green, blue);

    // A.24.5 - menggunakan associated function new (fields private)
    let random_color = model::Color::new(12, 25, 47);
    println!("{:#?} ", random_color);
}
