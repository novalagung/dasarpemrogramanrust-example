mod my_mod {
    
    pub fn run_the_app(note: &str) {
        println!("calling `my_mod::run_the_app()`. note {}", note);
    }
}

fn main() {
    my_mod::run_the_app("1st call");
    self::my_mod::run_the_app("2nd call");
}
