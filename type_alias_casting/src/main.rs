// A.27.1 - Type Alias
type Inch = u64;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
type Coordinate = Point;

fn main() {
    // A.27.2 - Casting tipe data & alias
    {
        let height: Inch = 6;
        println!("height: {height}");

        let height_in_u64 = height as u64;
        println!("height_in_u64: {height_in_u64}");
    }

    // A.27.3 - Casting antar tipe scalar
    {
        let number = 32;
        println!("number: {number}");

        let number_in_u8 = number as u8;
        println!("number_in_u8: {number_in_u8}");

        let number_in_f64 = number as f64;
        println!("number_in_f64: {number_in_f64}");

        let new_number = 23.4 as f32;
        println!("new_number: {new_number}");
    }

    // A.27.4 - Konsekuensi casting tipe numerik
    {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        println!("timestamp (u64): {timestamp}");
        println!("timestamp (as u16): {}", timestamp as u16);
        println!("from u16 back to u64: {}", (timestamp as u16) as u64);
    }

    // A.27.5 - Type alias bukan casting
    {
        let p = Point { x: 0, y: 10 };
        println!("p: {:?}", p);

        let mut q: Coordinate = p;
        q.x = 12;
        println!("q: {:?}", q);

        let r: Point = q;
        println!("r: {:?}", r);
    }
}
