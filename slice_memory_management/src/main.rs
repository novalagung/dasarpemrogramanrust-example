fn main() {
    {
        let data_arr = [1, 2, 3];
        println!("data_arr: {} {:?}", data_arr.len(), data_arr);
        let slice1 = &data_arr[1..];
        println!("slice1  : {} {:?}", slice1.len(), slice1);
        let slice2 = &data_arr[..2];
        println!("slice2  : {} {:?}", slice2.len(), slice2);

        println!();
    
        let data_vec = vec![1, 2, 3];
        println!("data_vec: {} {:?}", data_vec.len(), data_vec);
        let slice1 = &data_vec[1..];
        println!("slice1  : {} {:?}", slice1.len(), slice1);
        let slice2 = &data_vec[..2];
        println!("slice2  : {} {:?}", slice2.len(), slice2);

        println!();
    
        let data_str = String::from("sesuk prei jarene, mosokk");
        println!("data_str: {} {:?}", data_str.len(), data_str);
        let slice1 = &data_str[1..];
        println!("slice1  : {} {:?}", slice1.len(), slice1);
        let slice2 = &data_str[..2];
        println!("slice2  : {} {:?}", slice2.len(), slice2);
    }

    {
        let data_str = String::from("sesuk preiii");
        println!("data_str: {:?}", data_str);
        let slice1 = &data_str[8..];
        println!("slice1  : {:?}", slice1);
        let slice2 = &data_str[..5];
        println!("slice2  : {:?}", slice2);
    }

    {
        let mut numbers = [24, 12, 8, 7, 9, 2];
        println!("numbers: {:?}", numbers);
        let n1 = &mut numbers[2..];
        println!("n1     : {:?}", n1);
        n1[0] = 3022;
        println!("numbers: {:?}", numbers);
    }
}
