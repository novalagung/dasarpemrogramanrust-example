const SuperheroSuperman: &str = "superman";
const SuperheroOmniMan: &str = "omniman";
const SuperheroHyperion: &str = "hyperion";

#[non_exhaustive]
pub enum Superhero {
    Superman,
    OmniMan,
    Hyperion,
}

fn main() {
    let value = Superhero::Superman;

    match value {
        Superhero::Superman => println!("stronk"),
        Superhero::OmniMan => println!("stronk"),
        Superhero::Hyperion => println!("stronk"),
    }

    match value {
        Superhero::Superman => println!("stronk"),
        Superhero::OmniMan => println!("stronk"),
        _ => println!("stronk"),
    }

    match value {
        Superhero::Superman => println!("stronk"),
        Superhero::OmniMan => println!("stronk"),
    }
}