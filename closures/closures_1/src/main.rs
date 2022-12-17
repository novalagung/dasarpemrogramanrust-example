fn main() {
    let r = 10.0;

    let volume = calculate_circle_volume_v1(r);
    println!("{volume:.2}");

    fn calculate_circle_volume_v2(e: f64) -> f64 {
        const PI: f64 = 3.14;
        let volume = 4.0 / 3.0 * PI * e.powi(3);
        volume
    }
    
    let volume = calculate_circle_volume_v2(r);
    println!("{:.2}", volume);
}

fn calculate_circle_volume_v1(e: f64) -> f64 {
    const PI: f64 = 3.14;
    let volume = 4.0 / 3.0 * PI * e.powi(3);
    volume
}
