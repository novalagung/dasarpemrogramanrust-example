mod model;

fn main() {
    let red = model::Color::red();
    let green = model::Color::green();
    let blue = model::Color::blue();

    println!("{:#?} {:#?} {:#?}", red, green, blue);
    
    let random_color = model::Color::new(12, 25, 47);
    println!("{:#?} ", random_color);
}
