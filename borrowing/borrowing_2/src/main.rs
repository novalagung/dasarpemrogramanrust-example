fn main() {
    let mut fact_one = String::from("Arthas is the true lich king");
    println!("{:?}", fact_one);

    change_value(&mut fact_one);
    println!("{:?}", fact_one);

    {
        let fact_two = &mut fact_one;
        *fact_two = String::from("There must always be a lich king");
        println!("{:?}", fact_one);
    }

    if fact_one.contains("lich king") {
        let fact_three = &mut fact_one;
        *fact_three = String::from("Who is the real jailer?");
        println!("{:?}", fact_one);
    }

    for _ in 0..1 {
        let fact_four = &mut fact_one;
        *fact_four = String::from("Is it Zovaal or Primus?");
        println!("{:?}", fact_one);
    }
}

fn change_value(txt: &mut String) {
    *txt = String::from("Bolvar is better lich king");
}