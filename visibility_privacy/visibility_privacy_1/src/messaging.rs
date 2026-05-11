// A.28.2 - Default visibility
const SOME_MESSAGE: &str = "hello rust";

mod service_layer {
    pub fn some_black_magic() {
        println!("{}", crate::messaging::SOME_MESSAGE)
    }
}

pub fn say_hello() {
    service_layer::some_black_magic();
}
