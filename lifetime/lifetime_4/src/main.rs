// A.44.9 - Generic parameter + trait bounds + lifetime
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
    // A.44.9 - i32 comparison test
    {
        let x = 13;
        let y = 20;
        let result = find_greater_number(&x, &y);
        println!("result: {}", result);
    }

    // A.44.9 - f64 comparison test
    {
        let x = 3.14;
        let y = 2.11;
        let result = find_greater_number(&x, &y);
        println!("result: {}", result);
    }
}