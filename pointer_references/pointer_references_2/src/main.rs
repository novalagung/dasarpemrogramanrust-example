// A.33.4 - Mutable References (operator &mut)
fn main() {
    // Error: immutable reference cannot be modified
    // let mut number = 24;
    // println!("number: {}", number);
    // let pointer_number = &number;
    // *pointer_number = 12; // error

    let mut number = 24;
    println!("number: {}", number);

    let pointer_number: &mut i32 = &mut number;
    println!("pointer_number: {:p}", pointer_number);

    *pointer_number = 12;

    println!("*pointer_number: {}", *pointer_number);
    println!("number: {}", number);
}
