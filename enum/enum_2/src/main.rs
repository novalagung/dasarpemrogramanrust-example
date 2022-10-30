enum Food {
    PenyetanTerangBulan,
    PizzaNanas,
    EsKrimIkanMujaer,
    MiGorengKuah,
    MakananLainnya(String),
    MieSetan { level_pedas: i32, pakek_piring: bool }
}

fn main() {
    // let makanan_favorit: Food = Food::PenyetanTerangBulan;
    
    // let nasi_goreng = String::from("nasi goreng");
    // let makanan_favorit = Food::MakananLainnya(nasi_goreng);

    let makanan_favorit = Food::MieSetan { 
        level_pedas: 5,
        pakek_piring: false
    };
    
    match makanan_favorit {
        Food::PenyetanTerangBulan => {
            println!("your food taste is quite ... unique");
        },
        Food::PizzaNanas => {
            println!("it's morally wrong to have pineaple on top of pizza");
        },
        Food::EsKrimIkanMujaer => {
            println!("I don't know what to say. this should be illegal");
        },
        Food::MiGorengKuah => {
            println!("sometimes people do eat this, but it's ok");
        },
        Food::MakananLainnya(m) => {
            println!("do you like {m}? nice taste!");
        },
        Food::MieSetan { level_pedas, pakek_piring } => {
            if level_pedas > 3 {
                println!("mie setan lvl {} is too much!", level_pedas);
            } else {
                println!("mie setan lvl {} is perfect!", level_pedas);
            }
            
            if !pakek_piring {
                println!("how are you going to eat the food without a plate, huh?");
            }
        },
        _ => {
            println!("never heard about that food");
        }
    }
}