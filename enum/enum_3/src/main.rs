// A.26.8 - enum module & visibility
mod constants;

fn main() {
    let company = constants::Company::Apple;

    match company {
        constants::Company::Apple => {
            print!("apple")
        },
        _ => {
            print!("other than apple")
        },
    }
}
