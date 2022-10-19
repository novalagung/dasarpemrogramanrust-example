mod my_io;
mod my_number;

fn main() {
    println!("enter any number:");
    let message = my_io::read_entry();
    println!("your number: {}", message);

    let number = my_number::conversion_utility::string_to_number(message);
    let result = my_number::is_odd_number(number);
    println!("is odd number: {}", result);
}
