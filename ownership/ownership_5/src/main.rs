fn main() {
    // A.34.6 - Transfer ownership via parameter/argument
    let mut msg = String::from("hello rust");
    msg = say_hello(msg);
    println!("{:?}", msg);
}

fn say_hello(param: String) -> String {
    println!("{:?}", param);
    param
}
