fn main() {
    // A.6.1 - String literal atau &str
    // let y = "hello"

    // A.6.2 - escape characters using backslash
    let var2 = "hello \
    \"rust\" \
    and \
    \"world\"";
    println!("{}", var2);

    // A.6.3 - multiline string literal
    let var3 = "baris satu
baris dua
baris tiga";
    println!("{}", var3);

    // A.6.3 - multiline string literal (spaces are preserved)
    let var4 = "baris satu
    baris dua
    baris tiga";
    println!("{}", var4);

    // A.6.4 - raw string
    let var5 = r#"
    {
        "name": "tim drake",
        "gender": "male"
    }
"#;
    println!("{}", var5);

    // A.6.4 - raw string equivalent (using escape instead)
    let var6 = "
    {
        \"name\": \"cassandra cain\",
        \"gender\": \"female\"
    }
";
    println!("{}", var6);
}
