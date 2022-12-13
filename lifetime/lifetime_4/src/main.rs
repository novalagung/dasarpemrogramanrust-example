fn find_greater_number<'a, T>(
    x: &'a T,
    y: &'a T,
) -> &'a T
where
    T: std::cmp::PartialOrd,
{
    if x > y {
        x
    } else {
        y
    }
}

fn main() {

    {
        // i32 comparison test
        let x = 13;
        let y = 20;
        let result = find_greater_number(&x, &y);
        println!("result: {}", result);
    }
    
    {
        // f64 comparison test
        let x = 3.14;
        let y = 2.11;
        let result = find_greater_number(&x, &y);
        println!("result: {}", result);
    }
}