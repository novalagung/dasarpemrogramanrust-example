fn main() {
    // A.31.1 - Variable shadowing
    let some_data = "Hello";
    println!("{}", some_data);

    let some_data = 12;
    println!("{}", some_data);

    let some_data = "Rust!";
    println!("{}", some_data);

    let mut some_data = false;
    some_data = true;
    println!("{}", some_data);

    let some_data = 3.14;
    println!("{}", some_data);

    // A.31.2 - Shadowing pada block berbeda
    let name = "Orgrim Doomhammer";
    let mut race = "Orc";
    let mut number = 27;

    println!("{}\t {}\t {}", name, race, number);

    {
        let name = "Sylvanas Windrunner";
        race = "Undead";
        let number = 24;

        println!("{}\t {}\t {}", name, race, number);
    }

    println!("{}\t {}\t {}", name, race, number);
}
