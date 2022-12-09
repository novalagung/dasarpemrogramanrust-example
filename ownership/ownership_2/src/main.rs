fn main() {
    do_something();
}

fn do_something() {
    let mut k = String::from("hello");

    {
        let m = String::from("hello world");
        let n = String::from("from rust");
        k = n;
        
        println!("{:?}", m);
    }

    println!("{:?}", k);
}