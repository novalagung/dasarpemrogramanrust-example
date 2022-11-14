pub use self::service_layer::some_black_magic as say_hello;

const SOME_MESSAGE: &str = "hello rust";

mod service_layer {

    pub fn some_black_magic() {
        println!("{}", crate::messaging::SOME_MESSAGE)
    }
}