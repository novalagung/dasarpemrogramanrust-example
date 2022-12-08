fn do_something() {
    let some_data = "one";
    // ...
}

fn main() {
    let some_data = "two";
    // ...

    {
        let some_data = "three";
        // ...
    }

    do_something();

    if true {
        let some_data = "four";
        // ...
    }
}
