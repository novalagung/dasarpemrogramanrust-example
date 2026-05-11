fn main() {
    // A.9.1 - if keyword
    let number_a = 3;
    if number_a < 5 {
        println!("number_a adalah di bawah 5");
    }

    let result_a = number_a >= 5;
    if result_a {
        println!("result_a adalah di atas atau sama dengan 5");
    }

    // A.9.2 - if, else if, else
    let number_b = 3;
    if number_b == 2 {
        println!("number_b adalah 2");
    } else if number_b < 2 {
        println!("number_b adalah di bawah 2");
    } else {
        println!("number_b adalah di atas 2");
    }

    // A.9.3 - nested if
    let number_c = 10;
    if number_c > 6 {
        println!("selamat, anda lulus");

        if number_c == 10 {
            println!("dengan nilai sempurna");
        } else if number_c > 7 {
            println!("dengan nilai baik");
        } else {
            println!("dengan nilai cukup");
        }
    } else {
        println!("anda tidak lulus");

        if number_c < 4 {
            println!("belajar lagi ya");
        } else {
            println!("jangan malas belajar!");
        }
    }

    // A.9.4 - returning from if
    let number_d = 3;
    let result_d: bool;

    if number_d == 2 {
        result_d = true
    } else {
        result_d = false
    }

    println!("result_d adalah {result_d}");

    // A.9.4 - let if
    let number_d = 3;
    let result_d =
        if number_d == 2 {
            true
        } else {
            false
        };
    println!("result_d adalah {result_d}");

    // A.9.5 - let if with explicit type
    let number_e = 3;
    let result_e: &str = if number_e == 2 {
        "angka adalah 2"
    } else if number_e < 2 {
        "angka adalah di bawah 2"
    } else {
        "angka adalah di atas 2"
    };
    println!("angka adalah {result_e}");

    // A.9.5 - let if with explicit type (example 2)
    let max = 100.0;
    let string_f = "nilai minimum kelulusan";
    let result_f: f64 = if string_f == "nilai maksimum kelulusan" {
        max
    } else {
        max * 3.0 / 4.0
    };
    println!("angka adalah {result_f}");
}
