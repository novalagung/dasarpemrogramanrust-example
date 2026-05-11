// A.20.4 - submodule item: fungsi string_to_number
pub fn string_to_number(text: String) -> i32 {
    return text.parse::<i32>().unwrap();
}
