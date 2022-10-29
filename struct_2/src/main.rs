mod models;

fn main() {
    let ps5 = models::game::GamingConsole{
        name: String::from("PS 5")
    };
    println!("{:#?}", ps5);










    

    let red = models::color::Color(255, 255, 0);
    println!("{:#?}", red);
}
