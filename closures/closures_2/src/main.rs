
fn main() {
    // closure dengan 2 parameter tanpa return value
    let do_something_v1 = | a: i32, b: String | {
        // ...
    };

    // closure dengan 2 parameter dan return value bertipe tuple
    let do_something_v2 = | a: i32, b: String | -> (i32, bool) {
        // ...
        (0, false)
    };

    // closure tanpa parameter dan return value bertipe Vec<String>
    let do_something_v3 = | | -> Vec<String> {
        // ...
        Vec::new()
    };

    // closure tanpa parameter dan tanpa return value
    let do_something_v4 = | | {
        // ...
    };

    let get_pi = || 3.14;

    
    fn pow_v1(x: i32) -> i32 {
        x.pow(2)
    }
    let pow_v2 = |x: i32| -> i32 {
        x.pow(2)
    };
    let pow_v3 = |x: i32| {
        x.pow(2)
    };
    let pow_v4 = |x: i32| x.pow(2);

}
