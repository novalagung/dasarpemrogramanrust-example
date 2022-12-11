fn main() {
    // let number = 12;
    // println!("{:?}", number);

    // let text = String::from("hello");
    // println!("{:?}", text);

    let circle_one = Circle{raidus: 6};
    println!("{:?}", circle_one);
    println!("{}", circle_one);
}

struct Circle {
    raidus: i32,
}

impl std::fmt::Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle radius: {}", self.raidus)
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle radius: {}", self.raidus)
    }
}