use std::io;
use std::io::Write;

fn main() {
    print!("enter your name: ");
    let _ = io::stdout().flush();

    let name = read_entry();
    if name.is_empty() {
        panic!("unable to continue the program");
    }

    println!("hi {}", name);
}

pub fn read_entry() -> String {
    let mut message = String::new();
    let reader_res = io::stdin().read_line(&mut message);

    if reader_res.is_err() {
        return message;
    }

    message.trim().to_string()
}
