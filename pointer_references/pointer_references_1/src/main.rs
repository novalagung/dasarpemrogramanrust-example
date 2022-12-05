fn main() {
    let number = 24;
    println!("value: {:?}", number);

    let pointer_number = &number;
    println!("pointer: {:p}", pointer_number);

    let underlying_value = *pointer_number;
    println!("value: {:}", underlying_value);
}
