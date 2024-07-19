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

pub mod m1 {
    #[allow(missing_docs)]
    pub fn undocumented_one() -> i32 { 1 }

    #[warn(missing_docs)]
    pub fn undocumented_too() -> i32 { 2 }

    // #[deny(missing_docs)]
    // pub fn undocumented_end() -> i32 { 3 }
}

// #[forbid(missing_docs)]
// pub mod m3 {
//     #[allow(missing_docs)]
//     pub fn undocumented_too() -> i32 { 2 }
// }

fn main() {

    #[allow(unused_variables)]
    let name = "noval agung";

    say_hello();
}
