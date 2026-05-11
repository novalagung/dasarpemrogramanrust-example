fn main() {
    // A.34.7 - Clone data
    let msg = String::from("hello rust");
    say_hello(msg.clone());
    println!("{:?}", msg);
}

fn say_hello(param: String) {
    println!("{:?}", param);
}
