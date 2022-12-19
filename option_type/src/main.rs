fn main() {
    {
        let result1 = divider(10, 5);
        println!("result: {:?}", result1);
    
        let result2 = divider(10, 0);
        println!("result: {:?}", result2);
    }

    // {
    //     let result1 = divider(10, 5);
    //     match result1 {
    //         None    => println!("cannot divide by 0"),
    //         Some(x) => println!("result: {x}"),
    //     }

    //     let result2 = divider(10, 0);
    //     match result2 {
    //         None    => println!("cannot divide by 0"),
    //         Some(x) => {
    //             println!("result: {}", x)
    //         },
    //     }
    // }

    // {
    //     let result1 = divider(10, 5);
    //     if result1 != None {
    //         let number = result1.unwrap();
    //         println!("result: {}", number);
    //     }
    // }

    // {
    //     let result2 = divider(10, 0);
    //     let number = result2.unwrap();
    //     println!("result: {}", number);
    // }

    // {
    //     let result2 = divider(10, 0);
    //     let number = result2.unwrap_or_default();
    //     println!("result: {}", number);
    // }

    // {
    //     let result2 = divider(10, 0);
    //     let number = result2.unwrap_or(0);
    //     println!("result: {}", number);
    // }

    // {
    //     let result2 = divider(10, 0);
    //     let number = result2.unwrap_or_else(|| -> i32 { 0 });
    //     println!("result: {}", number);
    // }
}

fn divider(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }

    let result = a / b;
    return Some(result);
}
