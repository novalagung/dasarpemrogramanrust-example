fn main() {
    let msg = do_something();
    println!("{:?}", msg);
}
 
fn do_something() -> String {
    let mut k = String::from("hello");

    {
        let m = String::from("hello world");
        k = m;
    }

    return k;
}