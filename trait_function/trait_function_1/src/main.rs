fn main() {
    // Fn
    {
        let pow_number = |n: i32| n.pow(2);

        println!("pow_number(2): {}", pow_number(2));
        println!("pow_number(3): {}", pow_number(3));
        println!("pow_number(4): {}", pow_number(4));
    }
    {
        let result = do_something_with_number_v1(13, |d: i32| d * 2);
        println!("result: {:?}", result);
    }

    // FnMut
    {
        let mut x = 5;
        {
            let mut square_x = || x *= x;
            square_x();
        }
        println!("result: {}", x);
    }
    {
        let mut number = 1;
        do_something_with_number_v2(14, |x| number += x);
        println!("number: {number}");
    }

    // FnOnce
    {
        let mut x = 5;
        {
            let mut square_x = || x *= x;
            square_x();
        }
        println!("result: {}", x);
    }
    {
        let mut number = 1;
        do_something_with_number_v3(14, |x| number += x);
        println!("number: {number}");
    }
}

fn do_something_with_number_v1<F>(n: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    return f(n);
}

fn do_something_with_number_v2<F>(n: i32, mut f: F)
where
    F: FnMut(i32),
{
    f(n);
}

fn do_something_with_number_v3<F>(n: i32, f: F)
where
    F: FnOnce(i32),
{
    f(n);
}