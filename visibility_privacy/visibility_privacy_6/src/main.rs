pub mod outer_mod {

    pub mod inner_mod {

        // fungsi ini visibility scope-nya di current module scope,
        // yaitu `inner_mod`
        pub(self) fn say_hello() {
            println!("hello rust")
        }

        pub fn do_something() {
            say_hello();
        }
    }
}

fn main() {
    outer_mod::inner_mod::do_something();
}
