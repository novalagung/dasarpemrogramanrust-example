// A.43.3 - Keyword `static`
static PI: f64 = 3.14;

// A.43.4 / A.43.5 - Lifetime `'static` dan static item data literal
const VERSION: &str = "v1.2.3";
const BUILD_COUNTER: &i32 = &15;

fn main() {
    println!("PI: {:?}", PI);
    println!("VERSION: {:?}", VERSION);
}
