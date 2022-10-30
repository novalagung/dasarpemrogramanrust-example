enum GamingConsole {
    XBox,
    PlayStation,
    Switch
}

fn get_message(c: GamingConsole) -> String {
    match c {
        GamingConsole::XBox => {
            String::from("you got xbox? nice!")
        },
        GamingConsole::PlayStation => {
            String::from("playstation is one of the best console")
        },
        GamingConsole::Switch => {
            String::from("really love the portability of switch")
        }
    }
}

fn main() {
    let my_console = GamingConsole::PlayStation;
    let message = get_message(my_console);
    println!("{message}");
}
