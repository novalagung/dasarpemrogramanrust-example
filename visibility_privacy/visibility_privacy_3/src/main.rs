pub mod outer_mod {

    pub mod inner_mod {

        // fungsi say_hello berikut hanya bisa diakses dari dalam `outer_mod`.
        // pengaksesannya dari luar `outer_mod` menghasilkan error.
        pub(in crate::outer_mod) fn say_hello() {
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