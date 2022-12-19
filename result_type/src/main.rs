#[derive(Debug)]
enum MathError {
    DivisionByZero,
    InfinityNumber,
    OtherError,
}

fn main() {
    {
        let result1 = divider(10.0, 5.0);
        println!("result: {:?}", result1);
    
        let result2: Result<f64, MathError> = divider(10.0, 0.0);
        println!("result: {:?}", result2);
    }

    // {
    //     let result = divider(10.0, 5.0);
    //     match result {
    //         Err(m) => println!("ERROR! {:?}", m),
    //         Ok(r)        => println!("result: {r:.2}"),
    //     }
    // }

    // {
    //     let result = divider(10.0, 5.0);
    //     match result {
    //         Err(MathError::DivisionByZero) => println!("ERROR! unable to divide number by 0"),
    //         Err(MathError::InfinityNumber) => println!("ERROR! result is infinity number (∞)"),
    //         Err(_)                         => println!("ERROR! unknown error"),
    //         Ok(2.0)                        => println!("the result is 2"),
    //         Ok(x)                     => println!("result: {x:.2}"),
    //     }
    // }

    // {
    //     let result = divider(10.0, 5.0);
    //     if result.is_ok() {
    //         let number = result.unwrap();
    //         println!("result: {}", number);
    //     }
    // }

    // {
    //     let result = divider(10.0, 0.0);
    //     if result.is_err() {
    //         let err = result.as_ref().err();
    //         let message = err.unwrap();
    //         println!("error: {:?}", message);
    //     }
    //     if result.is_ok() {
    //         let data = result.as_ref().ok();
    //         let number = data.unwrap();
    //         println!("result: {:?}", number);
    //     }
    // }

    // {
    //     let result = divider(10.0, 0.0);
    //     let number = result.unwrap_or_default();
    //     println!("result: {}", number);
    //     // result: 0
    // }

    // {
    //     let result = divider(10.0, 0.0);
    //     let number = result.unwrap_or(0.0);
    //     println!("result: {}", number);
    //     // result: 0
    // }

    // {
    //     let result = divider(10.0, 0.0);
    //     let number = result.unwrap_or_else(|_| 0.0);
    //     println!("result: {}", number);
    //     // result: 0
    // }
}

fn divider(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        return Err(MathError::DivisionByZero);
    }

    let result = a / b;
    return Ok(result);
}