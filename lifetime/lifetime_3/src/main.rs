
// struct Book<'abc> {
//     title: &'abc str,
//     description: &'abc str,
// }

struct Book<'abc, 'def> {
    title: &'abc str,
    description: &'abc str,
    price: &'def i32,
}

impl<'abc, 'def> Book<'abc, 'def> {

    fn get_info(&self) -> String {
        let info = format!("{} (${}), {}", self.title, self.price, self.description);
        info
    }

    fn get_price(&self) -> &i32 {
        self.price
    }
}

fn get_book_price<'ghi>(book: &'ghi Book) -> &'ghi i32 {
    book.get_price()
}

fn main() {
    let book = Book {
        title: "The Silmarillion",
        description: "Good story, 10/10, would read again",
        price: &99,
    };

    println!("{}", book.get_info());
    println!("the price: {}", get_book_price(&book));
}