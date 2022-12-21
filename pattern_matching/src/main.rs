struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Black,
    White,
    Rgb(i32, i32, i32)
}

fn main() {
    // // matching literals
    // {
    //     let time = "morning";
    //     match time {
    //         "morning" => println!("isuk"),
    //         "afternoon" => println!("awan"),
    //         "evening" => println!("bengi"),
    //         _ => println!("mbuh kapan"),
    //     }
    // }

    // // matching literals
    // {
    //     let time = "morning";
    //     let time_but_in_javanese = match time {
    //         "morning" => "isuk",
    //         "afternoon" => "awan",
    //         "evening" => "bengi",
    //         _ => "mbuh kapan",
    //     };
    //     println!("{time_but_in_javanese}");
    // }

    // // matching enum value
    // {
    //     let value: Option<i32> = Option::Some(5);
    //     match value {
    //         Some(1) => println!("one"),
    //         Some(2) => println!("two"),
    //         Some(x) => println!("{x} greater than two"),
    //         _ => println!("none"),
    //     }
        
    //     match value {
    //         Some(1) => println!("one"),
    //         Some(2) => println!("two"),
    //         Some(x) => println!("{x} greater than two"),
    //         None    => println!("none"),
    //     }
    // }

    // // multiple patterns
    // {
    //     let value = 6;
    //     match value {
    //         1 | 2 => println!("one or two"),
    //         3..=5 => println!("three through five"),
    //         6 => println!("six"),
    //         _ => println!("other number"),
    //     }

    //     let value: Option<i32> = Some(5);
    //     match value {
    //         Some(1 | 2) => println!("one or two"),
    //         Some(3..=5) => println!("three through five"),
    //         Some(6) => println!("six"),
    //         Some(x) => println!("{x} greater than six"),
    //         _ => println!("none"),
    //     }
    // }


    // // Extra Conditionals with Match Guards
    // {
    //     let value = Some(4);
    //     let message = match value {
    //         Some(x) if x % 2 == 0 => format!("number {} is even", x),
    //         Some(x) => format!("number {} is odd", x),
    //         None => String::new(),
    //     };
    //     println!("{message}");
    // }

    // // if let
    // {
    //     let value: Option<i32> = Some(5);
    //     match value {
    //         Some(1) => println!("one"),
    //         Some(x) => println!("{x} greater than two"),
    //         _ => println!("none"),
    //     }

    //     let value = Some(5);
    //     if let Some(1) = value {
    //         println!("one");
    //     } else if let Some(x) = value {
    //         println!("{x} greater than two");
    //     } else {
    //         println!("none");
    //     }

    //     let value = 6;
    //     match value {
    //         1 | 2 => println!("one or two"),
    //         3..=5 => println!("three through five"),
    //         6 => println!("six"),
    //         _ => println!("other number"),
    //     }
        
    //     let value = Some(5);
    //     if let Some(1 | 2) = value {
    //         println!("one or two");
    //     } else if let Some(3..=5) = value {
    //         println!("three through five");
    //     } else if let Some(6) = value {
    //         println!("six");
    //     } else {
    //         println!("other number");
    //     }
    // }

    // while let
    // {
    //     loop {
    //         match gfg.next() {
    //             Some(x) => print!("{}", x),
    //             _ => break,
    //         }
    //     }
    // }

    // // struct destructuring
    // {
    //     let p = Point { x: 0, y: 7 };

    //     let Point { x, y } = p;
    //     println!("x: {x}");
    //     println!("y: {y}");

    //     match p {
    //         Point { x, y: 0 } => println!("x axis at {x}"),
    //         Point { x: 0, y } => println!("y axis at {y}"),
    //         Point { x, y }    => println!("axis: ({x}, {y})")
    //     }
    // }

    // // enum destructuring
    // {
    //     let color = Color::Rgb(0, 160, 255);

    //     if let Color::Rgb(r, g, b) = color {
    //         println!("r: {r}");
    //         println!("g: {g}");
    //         println!("b: {b}");
    //     }

    //     match color {
    //         Color::Rgb(r, g, b) => println!("r: {r}, g: {g}, b: {b}"),
    //         _ => println!("other color")
    //     }
    // }

    // // destructuring tuple
    // {
    //     let grades = ("A", "B", "C");

    //     let (grade_a, grade_b, grade_c) = grades;
    //     println!("grade_a: {grade_a}");
    //     println!("grade_b: {grade_b}");
    //     println!("grade_c: {grade_c}");

    //     match grades {
    //         (grade_a, grade_b, grade_c) => {
    //             println!("grade_a: {grade_a}");
    //             println!("grade_b: {grade_b}");
    //             println!("grade_c: {grade_c}");
    //         }
    //     }
    // }

    // // variable _
    // {
    //     let numbers = (2, 4, 32);

    //     let (_, second, _) = numbers;
    //     println!("second number: {second}");
    // }

    // // operator ..
    // {
    //     let numbers = (2, 4, 8, 16, 32);

    //     let (first, .., last) = numbers;
    //     println!("first number: {first}");
    //     println!("last number: {last}");

    //     let (first, ..) = numbers;
    //     println!("first number: {first}");

    //     let (.., last) = numbers;
    //     println!("last number: {last}");
    // }

    
    // @ bindings
    {
        let value = 3;
        match value {
            n @ (1 | 2) => println!("one or two ({n})"),
            n @ 3..=5 => println!("three through five ({n})"),
            6 => println!("six"),
            _ => println!("other number"),
        }
    }
}
