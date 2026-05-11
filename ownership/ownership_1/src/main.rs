fn main() {
    // A.34.4 - Copy semantics
    let x = 24;
    let y = x;
    println!("x: {:?}, y: {:?}", x, y);

    // Error: move semantics (String)
    // let a = String::from("hello rust");
    // let b = a;
    // println!("a: {:?}, b: {:?}", a, b);

    // Error: move semantics (struct)
    // #[derive(Debug)]
    // struct MyStruct;
    // let g = MyStruct{};
    // let h = g;
    // println!("g: {:?}, h: {:?}", g, h);
}
