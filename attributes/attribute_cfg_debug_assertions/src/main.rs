fn main() {
    #[cfg(debug_assertions)]
    {
        println!("debug mode. from attribute cfg")
    }

    #[cfg(not(debug_assertions))]
    {
        println!("release mode. from attribute cfg")
    }

    if cfg!(debug_assertions) {
        println!("debug mode. from macro cfg!()");
    } else  {
        println!("release mode. from macro cfg!()");
    }
}
