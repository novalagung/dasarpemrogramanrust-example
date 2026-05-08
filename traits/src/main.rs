fn main() {
    // let number = 12;
    // println!("{:?}", number);

    // let text = String::from("hello");
    // println!("{:?}", text);

    let circle_one = Circle{radius: 6};
    println!("{:?}", circle_one);
    println!("{}", circle_one);
}

struct Circle {
    radius: i32,
}

impl std::fmt::Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle radius: {}", self.radius)
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle radius: {}", self.radius)
    }
}