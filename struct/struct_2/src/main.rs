// A.23.8 & A.23.9 - struct & tuple struct property visibility
mod models;

fn main() {
    // A.23.8 - struct as module item
    let ps5 = models::game::GamingConsole {
        name: String::from("PS 5"),
    };
    println!("{:#?}", ps5);

    // A.23.9 - tuple struct as module item
    let red = models::color::Color(255, 255, 0);
    println!("{:#?}", red);
}
