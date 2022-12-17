

fn main() {
    {
        let str1 = String::from("Nokia 3310");
        println!("{str1}");

        let str2 = "iPhone 8".to_string();
        println!("{str2}");

        let str3 = String::new();
        println!("{str3}");

        let str4 = String::from_utf8(vec![78, 55, 51]).unwrap();
        println!("{str4}");
    }

    // {
    //     let mut str5 = String::new();
    //     println!("{str5}");
    //     // ""
     
    //     str5 = String::from("Pixel 5");
    //     println!("{str5}");
    //     // Pixel 5
    // }

    // {
    //     let mut str6 = String::from("Pixel 6");
    //     println!("{str6}");
    //     // Pixel 6
        
    //     str6.insert_str(0, "my phone");
    //     println!("{str6}");
    //     // my phonePixel 6
        
    //     str6.insert_str(8, " is ");
    //     println!("{str6}");
    //     // my phone is Pixel 6
    // }

    // {
    //     let mut str7 = String::from("3310");
    //     str7.insert(0, 'N');
    //     str7.insert(1, 'o');
    //     str7.insert(2, 'k');
    //     str7.insert(3, 'i');
    //     str7.insert(4, 'a');
    //     str7.insert(5, ' ');
    //     println!("{str7}"); // Nokia 3310
    // }

    // {
    //     let mut str8 = String::from("Pixel 6");
    //     println!("{str8}"); // Pixel 6

    //     str8.push_str(" is a good phone");
    //     println!("{str8}"); // Pixel 6 is a good phone
    // }

    // {
    //     let mut str8 = String::from("Pixel");
    //     println!("{str8}"); // Pixel

    //     str8.push(' ');
    //     str8.push('7');
    //     println!("{str8}"); // Pixel 7
    // }

    // {
    //     let str9 = String::from("my phone is Pixel 6");
    //     let str10 = str9.replace("Pixel 6", "Nokia 3310");
    //     println!("str9: {str9}");   // my phone is Pixel 6
    //     println!("str10: {str10}"); // my phone is Nokia 3310
    // }

    // {
    //     let mut str11 = String::from("Nokia 3310");
    //     str11.clear();
    //     println!("{str11}"); // ""
    // }

    // {
    //     let str11 = String::from("Nokia 3310");

    //     let is_exists = str11.contains("3310");
    //     println!("{is_exists}"); // true

    //     let is_exists = str11.contains("3315");
    //     println!("{is_exists}"); // false
    // }

    // {
    //     let str12 = String::from("iPhone");
    //     let str13 = String::from("12");
    //     let str14 = String::from("Pro");
    //     let str: String = [str12, str13, str14].join(" ");
    //     println!("{str}"); // iPhone 12 Pro
    // }
}
