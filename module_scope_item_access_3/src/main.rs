fn my_func() {
    println!("calling `my_func()`");
}

mod my_mod {
    
    pub fn my_func() {
        println!("calling `my_mod::my_func()`");
    }
    
    pub fn run_the_app() {
        println!("calling `my_mod::run_the_app()`");
        crate::my_func();
        self::my_func();
    }
}

fn main() {
    my_mod::run_the_app();
}