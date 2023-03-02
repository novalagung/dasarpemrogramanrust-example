mod util1;
mod util2;

#[path = "util3_mymodule.rs"]
mod util3;

fn main() {
    util1::say_hello();
    util2::say_hello();
    util3::say_hello();
}
