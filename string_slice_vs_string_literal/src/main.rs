fn main() {
    // A.46.1 - String custom type (String)
    {
        let str1 = String::from("Lisa Blackpink");
        println!("str1: {str1}");
    }

    {
        let bytes = vec![69, 108, 117, 118, 101, 105, 116, 105, 101, 32, 243, 159, 164, 152];
        let str2 = String::from_utf8(bytes).unwrap();
        println!("str2: {}", str2);
    }

    // A.46.2 - String literal (&str)
    {
        let str3 = "Helena Iren Michaelsen Epica";
        println!("str3: {str3}");
    }

    // A.46.3 - Konversi data string
    {
        let str4: String = String::from("Hiroyuki Sawano");
        let str4_slice1: &str = str4.as_str();
        println!("str4: {str4}");
        println!("str4_slice1: {str4_slice1}");
    }

    // {
    //     let mut str5: String = String::from("Hans Zimmer");
    //     let str5_slice1: &mut str = str5.as_mut_str();
    //     println!("str5: {str5}");
    //     println!("str5_slice1: {str5_slice1}");
    // }

    {
        let mut str5: String = String::from("Hans Zimmer");
        {
            let str5_slice1: &mut str = str5.as_mut_str();
            println!("str5_slice1: {str5_slice1}");
        }
        println!("str5: {}", str5);
    }

    {
        let str6: &str = "John Towner Williams";
        let str6_slice1: String = str6.to_string();
        println!("str6: {str6}");
        println!("str6_slice1: {str6_slice1}");
        println!("str6: {str6}");
    }
}
