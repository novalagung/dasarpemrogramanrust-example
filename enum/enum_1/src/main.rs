// A.26.1, A.26.2, A.26.3 - definisi enum & seleksi kondisi
const SuperheroSuperman: &str = "superman";
const SuperheroOmniMan: &str = "omniman";
const SuperheroHomelander: &str = "homelander";
const SuperheroHyperion: &str = "hyperion";

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

    // A.26.3 - seleksi kondisi pada konstanta (valid)
    if value1 == SuperheroSuperman {
        println!("hello superman!");
    }

    // A.26.3 - seleksi kondisi pada enum (error, tidak memiliki PartialEq)
    // if value3 == Superhero::Superman {
    //     println!("hello superman!");
    // }
}
