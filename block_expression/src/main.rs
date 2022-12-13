use rand::Rng;

fn main() {
    let x = 24;
    println!("x: {}", x);

    {
        let y = 12;
        let z = x + y;
        println!("(from block) y: {}", y);
        println!("(from block) z: {}", z);
    }

    let a: i32 = {
        let n = rand::thread_rng().gen_range(0..100);
        n * 2
    };

    println!("a: {}", a);

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
