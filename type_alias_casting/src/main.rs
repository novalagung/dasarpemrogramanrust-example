use std::time::{SystemTime, UNIX_EPOCH};

type Inch = u64;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
type Coordinate = Point;

fn main() {
    {
        let height: Inch = 6;
        println!("height: {height}");

        let height_in_u64 = height as u64;
        println!("height_in_u64: {height_in_u64}");
    }

    // {
    //     let p = Point{ x: 0, y: 10 };
    //     println!("p: {:?}", p);

    //     let mut q: Coordinate = p as Coordinate;
    //     q.x = 12;
    //     println!("q: {:?}", q);

    //     let r: Point = q as Point;
    //     println!("r: {:?}", r);

    //     println!("{:?}", q);
    // }

    // {
    //     let number = 32;
    //     println!("number: {number}");

    //     let number_in_u8 = number as u8;
    //     println!("number_in_u8: {number_in_u8}");

    //     let number_in_f64 = number as f64;
    //     println!("number_in_f64: {number_in_f64}");

    //     let new_number = 23.4 as f32;
    //     println!("new_number: {new_number}");

    //     // let number_in_char = (number as u8) as char;
    //     // println!("number_in_char: {number_in_char}");
    // }

    // {
    //     let timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    //     println!("timestamp (u64): {timestamp}");
    //     println!("timestamp (as u16): {}", timestamp as u16);
    //     println!("from u16 back to u64: {}", (timestamp as u16) as u64);
    // }
}
