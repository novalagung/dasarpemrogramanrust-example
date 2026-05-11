// A.21.4 - normal module declaration (inline submodules in utilities.rs)
mod utilities;

// A.21.4 - main
fn main() {
    let password = format!("zereth mortis {}", utilities::random::string(10));
    println!("raw password: {}", password);

    let hashed = utilities::password::hash(&password);
    println!("hashed password: {}", hashed);

    let is_valid = utilities::password::is_valid(&password, &hashed);
    println!("is password matched? {}", is_valid);
}
