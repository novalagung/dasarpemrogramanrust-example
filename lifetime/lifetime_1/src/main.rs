fn main() {
    let message = String::from("darkspear is better than zandalari");
    print_mesage(&message);

    let m = get_message();
    println!("the message: {m}");

    let n = get_number();
    println!("the number: {n}");
}

fn print_mesage(m: &String) {
    println!("the message: {m}")
}
 
fn get_message() -> String {
    let message = String::from("darkspear is better than zandalari");
    message
}

fn get_number<'my_lifetime>() -> &'my_lifetime i32 {
    &13
}
