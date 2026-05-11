fn main() {
    // A.44.5 - Lifetime pada parameter
    let res = do_something_v7("hello", "rust");
    println!("{res}");
}

// A.44.5 - do_something_v1: equivalent to v2 with lifetime elision
// fn do_something_v1<'a>(x: &'a str) -> &'a str {
//     x
// }

// A.44.5 - do_something_v4: return value is new data
// fn do_something_v4<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str {
//     "hello"
// }

// A.44.5 - do_something_v5: return y with matching lifetime
// fn do_something_v5<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'b str {
//     y
// }

// A.44.5 - do_something_v6: simplified from v5
// fn do_something_v6<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
//     y
// }

// A.44.5 - do_something_v7: function with same lifetime
fn do_something_v7<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        x
    } else {
        y
    }
}
