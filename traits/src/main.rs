fn main() {
    // A.36.1 - Konsep traits: external trait Debug pada tipe primitif
    let number = 12;
    println!("{:?}", number);

    let text = String::from("hello");
    println!("{:?}", text);

    // A.36.1 - Error demo: struct Circle tanpa implementasi Debug
    // fn main() {
    //     let circle_one = Circle{radius: 6};
    //     println!("{:?}", circle_one);
    // }
    //
    // struct Circle {
    //     radius: i32,
    // }

    // A.36.1 - Wrapper pattern: eksternal trait ke eksternal type
    let wrapped = Wrapper(vec![String::from("a"), String::from("b")]);
    println!("{}", wrapped);

    // A.36.1 - Local trait Message
    let s = String::from("hello");
    s.log();

    // A.36.2 - Implementasi trait Debug dan Display untuk Circle
    let circle_one = Circle { radius: 6 };
    println!("{:?}", circle_one);
    println!("{}", circle_one);

    // A.36.3 - Default implementation pada trait method
    let p = Person { name: String::from("Alice") };
    p.greet();
    p.introduce();
}

// A.36.1 - Wrapper pattern
struct Wrapper(Vec<String>);

// A.36.1 - Local trait Message
trait Message {
    fn log(&self);
}

impl Message for String {
    fn log(&self) {
        println!("{}", self);
    }
}

// A.36.2 - Struct Circle dengan implementasi Debug dan Display
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

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

// A.36.3 - Default implementation pada trait method
trait Speak {
    fn greet(&self) {
        println!("Hello from default implementation!");
    }

    fn introduce(&self);
}

struct Person {
    name: String,
}

impl Speak for Person {
    fn introduce(&self) {
        println!("My name is {}", self.name);
    }
}
