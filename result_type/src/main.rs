// A.40.1 - Enum MathError untuk custom error type
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    InfinityNumber,
    OtherError,
}

fn main() {
    // A.40.1 - Konsep Result
    {
        let result1 = divider(10.0, 5.0);
        println!("result: {:?}", result1);

        let result2: Result<f64, MathError> = divider(10.0, 0.0);
        println!("result: {:?}", result2);
    }

    // A.40.2 - Pattern matching pada tipe Result
    {
        let result = divider(10.0, 5.0);
        match result {
            Err(m) => println!("ERROR! {:?}", m),
            Ok(r)  => println!("result: {r:.2}"),
        }
    }

    // A.40.2 - Pattern matching dengan seleksi nilai spesifik
    {
        let result = divider(10.0, 5.0);
        match result {
            Err(MathError::DivisionByZero) => println!("ERROR! unable to divide number by 0"),
            Err(MathError::InfinityNumber) => println!("ERROR! result is infinity number (∞)"),
            Err(_)                         => println!("ERROR! unknown error"),
            Ok(2.0)                        => println!("the result is 2"),
            Ok(x)                          => println!("result: {x:.2}"),
        }
    }

    // A.40.2 - Tips pattern matching: return value
    {
        let result: f64 = match divider(10.0, 5.0) {
            Err(m) => {
                println!("ERROR! {:?}", m);
                0.0
            },
            Ok(r) => r,
        };
        println!("result: {:?}", result);
    }

    // A.40.3 - Method is_ok & unwrap
    {
        let result = divider(10.0, 5.0);
        if result.is_ok() {
            let number = result.unwrap();
            println!("result: {}", number);
        }
    }

    // A.40.3 - Method as_ref, is_err, err, ok
    {
        let result = divider(10.0, 0.0);
        if result.is_err() {
            let err = result.as_ref().err();
            let message = err.unwrap();
            println!("error: {:?}", message);
        }
        if result.is_ok() {
            let data = result.as_ref().ok();
            let number = data.unwrap();
            println!("result: {:?}", number);
        }
    }

    // A.40.3 - Method unwrap_or_default
    {
        let result = divider(10.0, 0.0);
        let number = result.unwrap_or_default();
        println!("result: {}", number);
    }

    // A.40.3 - Method unwrap_or
    {
        let result = divider(10.0, 0.0);
        let number = result.unwrap_or(0.0);
        println!("result: {}", number);
    }

    // A.40.3 - Method unwrap_or_else
    {
        let result = divider(10.0, 0.0);
        let number = result.unwrap_or_else(|_| 0.0);
        println!("result: {}", number);
    }

    // A.40.6 - Tipe Result<(), E>
    {
        let result = divide_and_print(10.0, 1.0);
    }
}

// A.40.1 - Fungsi divider dengan return type Result<f64, MathError>
fn divider(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        return Err(MathError::DivisionByZero);
    }

    let result = a / b;
    return Ok(result);
}

// A.40.6 - Fungsi divide_and_print dengan return type Result<(), MathError>
fn divide_and_print(a: f64, b: f64) -> Result<(), MathError> {
    let res = divider(a, b);
    match res {
        Err(m) => {
            println!("ERROR! {:?}", m);
            Err(m)
        },
        Ok(n) => {
            println!("result: {}", n);
            Ok(())
        },
    }
}
