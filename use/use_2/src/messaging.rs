// A.29.2 - Re-export item menggunakan pub use
pub use self::sub_module::say_hello_message as say_hello;

mod sub_module {
    pub fn say_hello_message() {
        println!("hello rust")
    }
}
