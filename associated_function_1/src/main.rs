#[derive(Debug)]
struct LegoSet {
    code: i32,
    name: String,
    category: String,
    age_minimum: i32,
}

impl LegoSet {

    fn new(code: i32, name: String, category: String, age_minimum: i32) -> Self {
        Self { code, name, category, age_minimum }
    }

    fn what_is_lego() {
        println!("Lego is a line of plastic construction toys")
    }
}

fn main() {
    let rough_terrain_crane = LegoSet{
        code: 42082,
        name: String::from("Rough Terrain Crane"),
        category: String::from("Technic"),
        age_minimum: 11,
    };
    println!("{:#?}", rough_terrain_crane);
    
    let xtreme_offroader = LegoSet::new(
        42099, 
        String::from("4X4 X-treme Off-Roader"), 
        String::from("Technic"), 
        11,
    );
    println!("{:#?}", xtreme_offroader);

    LegoSet::what_is_lego();
}
