fn main() {
    // A.48.3 - Mutable closure
    {
        let num = 5;
        let display = || println!("{num}");

        println!("{num}");
        display();
    }

    {
        let mut num = 5;

        let mut increase_by = |x: i32| {
            num += x
        };

        increase_by(10);
        println!("{num}");
    }

    // A.48.4 - Borrowing pada closure
    // {
    //     let mut num = 5;
    //     let mut increase_by = |x: i32| num += x;

    //     num += 5;
    //     increase_by(10);

    //     println!("{num}");
    // }

    {
        let mut num = 5;
        let increase_by = |num: &mut i32, x: i32| *num += x;

        num += 5;
        increase_by(&mut num, 10);

        println!("{num}");
    }

    // A.48.5 - Keyword move
    {
        let mut num = 5;
        let mut increase_by = move |x: i32| {
            num += x;
            println!("{num} (from closure)");
        };

        increase_by(10);
        println!("{num}");
    }
}
