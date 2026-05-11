// A.20.4 - definisi module my_io dan my_number
mod my_io;
mod my_number;

fn main() {
    // A.20.4 - submodule
    println!("enter any number:");
    let message = my_io::read_entry();
    println!("your number: {}", message);

    let number = my_number::conversion_utility::string_to_number(message);
    let result = my_number::is_odd_number(number);
    println!("is odd number: {}", result);
}
