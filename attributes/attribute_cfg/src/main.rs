#[cfg(target_os = "linux")]
mod util {

    pub fn say_hello() {
        println!("hello (from linux)")
    }
}

#[cfg(target_os = "windows")]
mod util {

    pub fn say_hello() {
        println!("hello (from windows)")
    }

    pub fn say_something() {
        println!("how are you")
    }
}


fn main() {
    util::say_hello();

    #[cfg(target_os = "windows")]
    {
        util::say_something();
    }

    if cfg!(debug_assertions) {
        println!("Debugging....")
    } else {
        println!("Release....")    
    }
    
    #[cfg(debug_assertions)]
    {
        println!("Simulate Load env file...");
    }
    
    
    #[cfg(not(debug_assertions))]
    {
        println!("🦀")
    }
}
