fn main() {
    let msg = String::from("hello rust");
    say_hello(msg.clone());
    println!("{:?}", msg);
}
 
fn say_hello(param: String) {
    println!("{:?}", param);
}