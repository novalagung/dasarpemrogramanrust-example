// A.20.3 - module item: fungsi string_to_number dan is_odd_number
pub fn string_to_number(text: String) -> i32 {
    return text.parse::<i32>().unwrap();
}

pub fn is_odd_number(number: i32) -> bool {
    number % 2 != 0
}
