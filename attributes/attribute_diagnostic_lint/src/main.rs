#[allow(unused_imports)]
use std::fmt::Display;

#[deprecated]
fn say_hello() {
    println!("hello")
}

#[allow(dead_code)]
fn say_something() {
    println!("how are you")
}

fn main() {

    #[allow(unused_variables)]
    let name = "noval agung";

    say_hello();
}
