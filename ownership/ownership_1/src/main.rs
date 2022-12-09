fn main() {
    let x = 24;
    let y = x;
    println!("x: {:?}, y: {:?}", x, y);

    let a = String::from("hello rust");
    let b = a;
    println!("a: {:?}, b: {:?}", a, b);

    #[derive(Debug)]
    struct MyStruct;
    let g = MyStruct{};
    let h = g;
    println!("g: {:?}, h: {:?}", g, h);
}
