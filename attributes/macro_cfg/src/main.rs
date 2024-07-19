fn main() {
    #[cfg(target_os = "linux")]
    {
        println!("hello linux. from attribute cfg")
    }

    #[cfg(target_os = "windows")]
    {
        println!("hello windows. from attribute cfg")
    }

    if cfg!(target_os = "linux") {
        println!("hello linux. from macro cfg!()");
    } else if cfg!(target_os = "windows") {
        println!("hello windows. from macro cfg!()");
    }
}
