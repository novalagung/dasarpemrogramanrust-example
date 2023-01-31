const SuperheroSuperman: &str = "superman";
const SuperheroOmniMan: &str = "omniman";
const SuperheroHyperion: &str = "hyperion";

#[derive(PartialEq, Debug)]
enum Superhero {
    Superman,
    OmniMan,
    Hyperion,
}

impl std::fmt::Display for Superhero {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

// impl PartialEq for Superhero {

//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Superhero::Superman, Superhero::Superman) => true,
//             (Superhero::OmniMan, Superhero::OmniMan) => true,
//             (Superhero::Hyperion, Superhero::Hyperion) => true,
//             _ => false,
//         }
//     }
// }

fn main() {
    let value: Superhero = Superhero::Superman;

    if value == Superhero::Superman {
        println!("hello superman!");
    }

    println!("{value} (via `Display` trait)");
    println!("{value:#?} (via `Debug` trait)");
}
