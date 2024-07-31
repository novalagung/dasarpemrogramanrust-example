fn main() {
    do_some_math();
}

fn do_some_math() {
    let result1 = divider(10.0, 5.0);
    match result1 {
        Err(m) => println!("ERROR! {:?}", m),
        Ok(r) => println!("result: {r:.2}"),
    }
    
    let result2 = divider(10.0, 0.0);
    match result2 {
        Err(m) => println!("ERROR! {:?}", m),
        Ok(r) => println!("result: {r:.2}"),
    }
    
    let result3 = divider(10.0, 2.0);
    match result3 {
        Err(m) => println!("ERROR! {:?}", m),
        Ok(r) => println!("result: {r:.2}"),
    }
}

fn divider(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        return Err("division by zero error");
    }

    let result = a / b;
    return Ok(result);
}
