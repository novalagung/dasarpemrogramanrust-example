fn main() {
    let res = do_something_v7("hello", "rust");
    println!("{res}");
}

// fn do_something_v1<'a>(x: &'a str) -> &'a str {
//     x
// }

// fn do_something_v4<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str {
//     "hello"
// }

// fn do_something_v5<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'b str {
//     y
// }

// fn do_something_v6<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
//     y
// }

fn do_something_v7<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        x
    } else {
        y
    }
}
