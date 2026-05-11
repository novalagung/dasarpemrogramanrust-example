fn main() {
    // A.12.1 - for in with range 0..5
    for i in 0..5 {
        println!("{i}");
    }

    // A.12.1 - for in with range 0..=5
    for i in 0..=5 {
        println!("{i}");
    }

    // A.12.2 - label on for in
    'perulangan: for i in 0..=5 {
        if i > 3 {
            println!("perulangan dihentikan paksa pada iterasi {i}");
            break 'perulangan;
        }

        println!("{i}");
    }

    // A.12.3 - for in on array
    let array = ["jason", "grayon", "drake", "damian"];
    for name in array {
        println!("{name}");
    }
}
