pub mod outer_mod_one {

    pub mod inner_mod {

        // fungsi ini visibility scope-nya di level crate
        pub(crate) fn say_hello() {
            println!("hello rust")
        }
    }

    pub fn do_something() {
        inner_mod::say_hello();
    }
}

pub mod outer_mod_two {

    pub fn do_something() {
        crate::outer_mod_one::inner_mod::say_hello();
    }
}

fn main() {
    outer_mod_one::inner_mod::say_hello();
    outer_mod_one::do_something();
    outer_mod_two::do_something();
}
