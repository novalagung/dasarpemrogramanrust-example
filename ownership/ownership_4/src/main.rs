fn main() {
    // A.34.6 - Transfer ownership via return value
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
