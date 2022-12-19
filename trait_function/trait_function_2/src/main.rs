fn main() {
    let result = do_something_with_number_v1(13, |d: i32| d * 2);
    println!("result: {result}");
    
    let result = do_something_with_number_v1(13, double);
    println!("result: {result}");
    
    let result = do_something_with_number_v1(13, pow_number);
    println!("result: {result}");
}

fn do_something_with_number_v1<F>(n: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    return f(n);
}

fn double(d: i32) -> i32 {
    d * 2
}

fn pow_number(d: i32) -> i32 {
    d.pow(2)
}