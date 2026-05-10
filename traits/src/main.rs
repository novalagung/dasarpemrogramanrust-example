fn main() {
    // let number = 12;
    // println!("{:?}", number);

    // let text = String::from("hello");
    // println!("{:?}", text);

    let circle_one = Circle { radius: 6 };
    println!("{:?}", circle_one);
    println!("{}", circle_one);

    let wrapped = Wrapper(vec![String::from("a"), String::from("b")]);
    println!("{}", wrapped);

    let s = String::from("hello");
    s.log();
}

struct Circle {
    radius: i32,
}

struct Wrapper(Vec<String>);

trait Message {
    fn log(&self);
}

impl Message for String {
    fn log(&self) {
        println!("{}", self);
    }
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

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
