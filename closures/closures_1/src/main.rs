// A.48.1 - Konsep Closures
fn calculate_circle_volume_v1(e: f64) -> f64 {
    const PI: f64 = 3.14;
    let volume = 4.0 / 3.0 * PI * e.powi(3);
    volume
}

fn main() {
    let calculate_circle_volume_v2 = |e: f64| -> f64 {
        const PI: f64 = 3.14;
        let volume = 4.0 / 3.0 * PI * e.powi(3);
        volume
    };

    let r = 10.0;
    let volume = calculate_circle_volume_v1(r);
    println!("{volume:.2}");

    let volume = calculate_circle_volume_v2(r);
    println!("{:.2}", volume);

    // formatted print {:.n}
    let pi = 3.1415926535;

    println!("{:.4}", pi);
    println!("{pi:.4}");
}
