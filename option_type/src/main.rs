fn main() {
    // A.39.1 - Konsep Option
    {
        let result1 = divider(10, 5);
        println!("result: {:?}", result1);

        let result2: Option<i32> = divider(10, 0);
        println!("result: {:?}", result2);
    }

    // A.39.2 - Pattern matching pada tipe Option
    {
        let result1 = divider(10, 5);
        match result1 {
            None    => println!("cannot divide by 0"),
            Some(x) => println!("result: {x}"),
        }

        let result2 = divider(10, 0);
        match result2 {
            None    => println!("cannot divide by 0"),
            Some(x) => {
                println!("result: {}", x)
            },
        }
    }

    // A.39.2 - Pattern matching dengan seleksi nilai spesifik
    {
        let result1 = divider(10, 5);
        match result1 {
            None         => println!("cannot divide by 0"),
            Some(2)      => println!("the result is 2"),
            Some(x) => println!("result: {x}"),
        }
    }

    // A.39.2 - Tips pattern matching: return value
    {
        let result = match divider(10, 0) {
            None => {
                println!("cannot divide by 0");
                0
            },
            Some(x) => x,
        };

        println!("result: {:?}", result);
    }

    // A.39.3 - Method is_some, is_none, unwrap
    {
        let result1 = divider(10, 5);
        if result1 != None {
            let number = result1.unwrap();
            println!("result: {}", number);
        }
        if result1.is_some() {
            let number = result1.unwrap();
            println!("result: {}", number);
        }
        if !result1.is_none() {
            let number = result1.unwrap();
            println!("result: {}", number);
        }
    }

    // A.39.3 - Method unwrap (error pada None)
    // {
    //     let result2 = divider(10, 0);
    //     let number = result2.unwrap();
    //     println!("result: {}", number);
    // }

    // A.39.3 - Method unwrap_or_default
    {
        let result2 = divider(10, 0);
        let number = result2.unwrap_or_default();
        println!("result: {}", number);
    }

    // A.39.3 - Method unwrap_or
    {
        let result2 = divider(10, 0);
        let number = result2.unwrap_or(0);
        println!("result: {}", number);
    }

    // A.39.3 - Method unwrap_or_else
    {
        let result2 = divider(10, 0);
        let number = result2.unwrap_or_else(|| -> i32 { 0 });
        println!("result: {}", number);
    }
}

// A.39.1 - Fungsi divider dengan return type Option<i32>
fn divider(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }

    let result = a / b;
    return Some(result);
}
