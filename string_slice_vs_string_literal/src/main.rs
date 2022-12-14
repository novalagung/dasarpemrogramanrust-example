fn main() {
    // {
    //     let str1 = String::from("Lisa BlackPink");
    //     println!("str1: {str1}");
    // }

    // {
    //     let bytes = vec![69, 108, 117, 118, 101, 105, 116, 105, 101, 32, 240, 159, 164, 152];
    //     let str2 = String::from_utf8(bytes)
    //     println!("str2: {}", str2);
    // }

    // {
    //     let str3 = "Helena Iren Michaelsen Epica";
    //     println!("str3: {str3}");
    // }

    // {
    //     let str4: String = String::from("Hiroyuki Sawano");
    //     let str4_slice1: &str = str4.as_str();
    //     println!("str4: {str4}");
    //     println!("str4_slice1: {str4_slice1}");
    // }

    // {
    //     let mut str5 = String::from("Hans Zimmer");
    //     {
    //         let str5_slice1 = str5.as_mut_str();
    //         println!("str5_slice1: {str5_slice1}");
    //     }
    //     println!("str5: {}", str5);
    // }

    {
        let str6: &str = "John Towner Williams";
        let mut str6_slice1: String = str6.to_string();
        str6_slice1 = String::from("sdfsdfsdf");
        println!("str6: {str6}");
        println!("str6_slice1: {str6_slice1}");
        println!("str6: {str6}");
    }
}
