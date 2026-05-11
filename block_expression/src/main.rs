use rand::Rng;

fn main() {
    // A.30.1 - Konsep block expression
    let x = 24;
    println!("x: {}", x);

    {
        let y = 12;
        let z = x + y;
        println!("(from block) y: {}", y);
        println!("(from block) z: {}", z);
    }

    // Error: z is out of scope
    // let x = 24;
    // {
    //     let y = 12;
    //     let z = x + y;
    // };
    // println!("z: {}", z); // error

    // A.30.2 - Return value block
    let a: i32 = {
        let n = rand::thread_rng().gen_range(0..100);
        n * 2
    };

    println!("a: {}", a);

    // A.30.3 - Nested block
    {
        let b = 12;
        let mut total = 0;

        {
            let c = 13;

            {
                let d = 14;
                total = b + c + d;
            }
        }

        println!("{total}")
    }

    // A.30.4 - Labeled block
    {
        let mut total = 24;

        'append_with_even_number: {
            let n = rand::thread_rng().gen_range(0..100);

            if n % 2 == 1 {
                break 'append_with_even_number
            }

            total = n
        }

        println!("{total}");
    }
}
