fn main() {
    let mut number = 24;
    println!("number: {}", number);

    let pointer_number = &mut number;
    println!("pointer_number: {:p}", pointer_number);

    *pointer_number = 12;

    println!("*pointer_number: {}", *pointer_number);
    println!("number: {}", number);
}
