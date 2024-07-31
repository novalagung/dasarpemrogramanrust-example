fn main() {
    do_some_math();
}

fn do_some_math() -> Result<f64, &'static str> {
    let r1 = divider(10.0, 5.0)?;
    println!("result: {r1:.2}");
    
    let r2 = divider(10.0, 0.0)?;
    println!("result: {r2:.2}");
    
    let r3 = divider(10.0, 2.0)?;
    println!("result: {r3:.2}");

    return Ok(0.0)
}

fn divider(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        return Err("division by zero error");
    }

    let result = a / b;
    return Ok(result);
}
