use rand::Rng;

fn main() {
    let mut number = 24;
    println!("number: {}", number);

    for _ in 0..=5 {
        change_value(&mut number);
        println!("number: {}", number);
    }
}

fn change_value(n: &mut i32) {
    *n = generate_random_number()
}

fn generate_random_number() -> i32 {
    rand::thread_rng().gen_range(0..100)
}