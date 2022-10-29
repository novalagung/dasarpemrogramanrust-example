mod lego;

fn main() {
    let rough_terrain_crane = lego::LegoSet{
        code: 42082,
        name: String::from("Rough Terrain Crane"),
        category: String::from("Technic"),
        age_minimum: 11,
    };
    println!("{:#?}", rough_terrain_crane);
    
    let xtreme_offroader = lego::LegoSet::new(
        42099, 
        String::from("4X4 X-treme Off-Roader"), 
        String::from("Technic"), 
        11,
    );
    println!("{:#?}", xtreme_offroader);
}
