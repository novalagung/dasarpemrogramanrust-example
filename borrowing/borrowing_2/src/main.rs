fn main() {
    // A.35.5 - Borrowing valid/invalid reference
    let mut fact_one = String::from("Arthas is the true lich king");
    println!("{:?}", fact_one);

    change_value(&mut fact_one);
    println!("{:?}", fact_one);

    // A.35.6 - Borrowing pada block
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

    // A.35.7 - Owner dan borrower data literal
    {
        let number = 12;
        let a = &number;

        let text = String::from("hello");
        let b = &text;

        println!("{:?} {:?}", a, b);

        let c = &24;
        let d = &false;
        let e = &String::from("rust");

        println!("{:?} {:?} {:?}", c, d, e);
    }

    // A.35.8 - Borrowing pada macro `println`
    {
        let str1 = String::from("luwe");
        println!("{:?}", str1);

        let str2 = String::from("ngelak");
        println!("{:?}", &str2);
    }
}

fn change_value(txt: &mut String) {
    *txt = String::from("Bolvar is better lich king");
}
