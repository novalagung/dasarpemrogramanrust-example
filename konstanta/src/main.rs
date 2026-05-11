static NUMBER: i32 = 18;

fn main() {
    // A.7.1 - const keyword
    const LABEL: &str = "nilai hasil pembagian adalah:";
    const RESULT: f32 = 22.0 / 7.0;
    println!("{} {}", LABEL, RESULT);

    // A.7.2 - static keyword
    println!("{}", NUMBER);
}
