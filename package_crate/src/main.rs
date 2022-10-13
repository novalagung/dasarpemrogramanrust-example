use rand::Rng;

fn generate_random_number() -> i32 {
    rand::thread_rng().gen_range(0..100)
}

fn main() {
    for i in 0..5 {
        println!("random number {}: {}", i, generate_random_number());
    }
}
