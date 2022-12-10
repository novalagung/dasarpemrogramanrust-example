fn main() {
    let data: &String;
    {
        let s = String::from("hello rust");
        data = &s;
    }

    println!("{:?}", data)
}
