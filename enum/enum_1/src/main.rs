// definisi konstanta
const SuperheroSuperman: &str = "superman";
const SuperheroOmniMan: &str = "omniman";
const SuperheroHomelander: &str = "homelander";
const SuperheroHyperion: &str = "hyperion";

// definisi enum
enum Superhero {
    Superman,
    OmniMan,
    Homelander,
    Hyperion,
}

fn main() {
    let value1: &str = SuperheroSuperman;
    let value2 = SuperheroOmniMan;
    let value3: Superhero = Superhero::Superman;
    let value4 = Superhero::OmniMan;

    if value1 == SuperheroSuperman {
        println!("hello superman!");
    }

    // baris dibawah ini menghasilkan error
    if value3 == Superhero::Superman {
        println!("hello superman!");
    }
}
