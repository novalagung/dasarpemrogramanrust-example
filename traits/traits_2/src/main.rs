use std::fmt::{write, Display};

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("{:?}", self.0))
    }
}

fn main() {
    let v = Wrapper(vec![String::from("a"), String::from("b")]);
    println!("{}", v)
}
