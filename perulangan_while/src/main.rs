use std::thread::sleep;
use std::time::Duration;

fn main() {
    // A.10.1 - while keyword
    let mut i = 0;
    let max = 5;

    while i < max {
        println!("nilai: {i}");
        i += 1;
    }

    // A.10.2 - nested while
    let mut i = 0;
    let max = 5;

    while i < max {
        let mut j = 0;
        let max_inner = i;

        while j <= max_inner {
            print!("* ");
            j += 1;
        }

        println!();
        i += 1;
    }

    // A.10.2 - macro print
    print!("* ");
    print!("* ");
    print!("* ");
    print!("* ");
    println!();

    // A.10.4 - while loop with delay
    let mut i = 0;
    let max = 5;

    while i < max {
        println!("nilai: {i}");
        i += 1;

        sleep(Duration::from_secs(1));
    }
}
