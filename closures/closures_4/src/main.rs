fn main() {
    {
        let my_closure = do_something();
        println!("hello (from main)");
        my_closure();
    }
    
    {
        let my_closure = do_something_v2();
        let message = my_closure(123, "hello rust".to_owned());
        println!("{message} (from main)");
    }
}

fn do_something() -> impl Fn() {
    println!("hello (from do_something)");

    return || {
        println!("hello (from closure)");
    };
}

fn do_something_v2() -> impl Fn(i32, String) -> String {
    println!("hello (from do_something_v2)");

    return |a: i32, b: String| -> String {
        let message = format!("{b} {a}");
        message
    };
}
