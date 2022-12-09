fn main() {
    let mut msg = String::from("hello rust");
    msg = say_hello(msg);
    println!("{:?}", msg);
}
 
fn say_hello(param: String) -> String {
    println!("{:?}", param);
    param
}