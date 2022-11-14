pub mod outer_mod {

    pub mod inner_mod {

        // fungsi ini visibility scope-nya di parent module scope,
        // yaitu `outer_mod`
        pub(super) fn say_hello() {
            println!("hello rust")
        }
    }

    pub fn do_something() {
        inner_mod::say_hello();
    }
}

fn main() {
    outer_mod::do_something();
}
