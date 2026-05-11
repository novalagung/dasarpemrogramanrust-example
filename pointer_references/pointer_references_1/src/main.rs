fn main() {
    // A.33.2 - Reference (operator &)
    let number: i32 = 24;
    println!("value: {:?}", number);

    let pointer_number: &i32 = &number;
    println!("pointer: {:p}", pointer_number);

    // A.33.3 - Dereference (operator *)
    let underlying_value = *pointer_number;
    println!("value: {:}", underlying_value);
}
