fn main() {
    // A.8.1 - arithmetic operators
    let (num1, num2) = (12, 4);

    let value_addition = num1 + num2;
    println!("{} + {} = {}", num1, num2, value_addition);

    let value_sub = num1 - num2;
    println!("{} - {} = {}", num1, num2, value_sub);

    let value_mut = num1 * num2;
    println!("{} * {} = {}", num1, num2, value_mut);

    let value_div = num1 / num2;
    println!("{} / {} = {}", num1, num2, value_div);

    let value_mod = num1 % num2;
    println!("{} % {} = {}", num1, num2, value_mod);

    // A.8.2 - comparison operators
    let number_a = 12;
    let number_b = 24;

    let res_one = number_a == number_b;
    println!("res_one: {res_one}");

    let res_two = number_a != number_b;
    println!("res_two: {res_two}");

    let res_three = number_a > number_b;
    println!("res_three: {res_three}");

    let res_four = number_a < number_b;
    println!("res_four: {res_four}");

    let res_five = number_a >= number_b;
    println!("res_five: {res_five}");

    let res_six = number_a <= number_b;
    println!("res_six: {res_six}");

    // A.8.3 - negation operators
    let (value_left, value_right) = (12, -12);
    let res_one = -value_left == value_right;
    let res_two = !(value_left == value_right);
    println!("{res_one} {res_two}");

    // A.8.4 - logic / bool operators
    let (bool_left, bool_right) = (false, true);
    println!("AND result: {}", bool_left && bool_right);
    println!("OR result: {}", bool_left || bool_right);

    // A.8.4 - whitespace character tab \t
    println!("AND result\t: {}", bool_left && bool_right);
    println!("OR result\t: {}", bool_left || bool_right);
}
