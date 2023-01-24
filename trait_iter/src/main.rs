fn main() {
    {
        // array
        let data_arr = ["a", "b", "c", "d"];
        for e in &data_arr {
            print!("{e:?} ")
        }

        println!();

        // vector
        let data_vec = vec!["a", "b", "c", "d"];
        for e in &data_vec {
            print!("{e:?} ")
        }

        println!();

        // slice from vector
        let data_vec = vec!["a", "b", "c", "d"];
        let data_slice_vec = &data_vec[..];
        for e in data_slice_vec {
            print!("{e:?} ")
        }

        println!();
    }

    // {
    //     // array
    //     let data_arr = ["a", "b", "c", "d"];
    //     let iterator_arr = data_arr.iter();
    //     iterator_arr.for_each(|e| {
    //         print!("{e:?} ")
    //     });

    //     println!();

    //     // vector
    //     let data_vec = vec!["a", "b", "c", "d"];
    //     data_vec.iter().for_each(|e| {
    //         print!("{e:?} ")
    //     });

    //     println!();

    //     // slice from vector
    //     let data_vec = vec!["a", "b", "c", "d"];
    //     let data_slice_vec = &data_vec[..];
    //     data_slice_vec.iter().for_each(|e| {
    //         print!("{e:?} ")
    //     });

    //     println!();

    //     // slice from String
    //     let data_str = "abcd".to_string();
    //     let data_borrow_str = &data_str;
    //     data_borrow_str.chars().for_each(|e| {
    //         print!("{e:?} ")
    //     });

    //     println!();
    // }

    // {
    //     let data_arr = [1, 3, 5];

    //     let doubles: Vec<i32> = data_arr
    //         .iter()
    //         .map(|e| e * e)
    //         .collect();

    //     println!("{doubles:?}");
    // }

    // {
    //     let data_vec = vec!["1", "2", "3", "4", "a"];

    //     let numbers: Vec<i32> = data_vec
    //         .iter()
    //         .map(|e| -> i32 {
    //             match e.parse::<i32>() {
    //                 Ok(n) => n,
    //                 Err(_) => 0,
    //             }
    //         })
    //         .filter(|e| *e > 0 && *e % 2 == 0)
    //         .rev()
    //         .collect::<Vec<i32>>();

    //     println!("{numbers:?}");
    // }

    // {
    //     let data_vec = vec![1, 2, 3, 4];
        
    //     data_vec.iter().for_each(|d| {
    //         println!("{}", *d * 2)
    //     });
        
    //     for d in &data_vec {
    //         println!("{}", *d * 2)
    //     }
    // }

    // {
    //     let mut data_vec = vec![1, 2, 3, 4];
        
    //     data_vec.iter_mut().for_each(|d| {
    //         *d = *d * 2;
    //     });
    //     println!("{:?}", data_vec);
        
    //     for d in &mut data_vec {
    //         *d = *d * 2;
    //     }
    //     println!("{:?}", data_vec);
    // }

    // {
    //     let data_vec = vec![1, 2, 3, 4];
        
    //     data_vec.into_iter().for_each(|d| {
    //         println!("{}", d * 2)
    //     });
        
    //     // statement di bawah ini menghasilkan error,
    //     // karena ownership `data_vec` telah berpindah setelah method `into_iter` dipanggil

    //     // println!("{}", data_vec)
    // }

    // {
    //     let data_vec: Vec<i32> = vec![1, 2, 3, 4];

    //     let result: Vec<&i32> = data_vec.iter().collect();
    //     println!("{:?}", result);
    // }

    // {
    //     let data_vec: Vec<i32> = vec![1, 2, 3, 4];

    //     let result: Vec<&i32> = data_vec.iter().collect();
    //     println!("{:?}", result);
    // }

    // {
    //     let data_vec: Vec<i32> = vec![1, 2, 3, 4];

    //     let result1: Vec<&i32> = data_vec.iter().collect();
    //     println!("{:?}", result1);

    //     let result2: Vec<i32> = data_vec.iter().map(|d: &i32| -> i32 { *d }).collect();
    //     println!("{:?}", result2);
    // }

    // {
    //     let data_vec = vec![1, 2, 3, 4];
    //     println!("{:?}", data_vec);

    //     let result: Vec<&i32> = data_vec.iter().rev().collect();
    //     println!("{:?}", result);
    // }

    // {
    //     let data_vec = vec![1, 2, 3, 4];

    //     let odd: Vec<&i32> = data_vec.iter().filter(|d| *d % 2 != 0).collect();
    //     println!("odd numbers: {:?}", odd);

    //     let even: Vec<&i32> = data_vec.iter().filter(|d| *d % 2 == 0).collect();
    //     println!("even numbers: {:?}", even);
    // }

    // {
    //     let data_vec = vec![1, 2, 3, 4];
    //     let result: Vec<i32> = data_vec.iter().cloned().collect();
    //     println!("{:?}", result);
    // }

    // {
    //     let data_vec1 = vec![1, 2, 3, 4];
    //     let data_vec2 = vec![1, 2, 3];
    //     let result = data_vec1.iter().cmp(data_vec2.iter());
    //     println!("{:?}", result.is_eq());
    // }

    // {
    //     let data_vec = vec![1, 2, 3, 4];
    //     let length = data_vec.iter().count();
    //     println!("{:?}", length);
    // }

    // {
    //     let data_vec1 = vec![1, 2, 3, 4];
    //     let data_vec2 = vec![1, 2, 3];
    //     let result = data_vec1.iter().eq(data_vec2.iter());
    //     println!("{:?}", result);
    // }

    // {
    //     let data_vec = vec![1, 2, 3, 4];
    //     let result = match data_vec.iter().find(|d: &&i32| **d == 4) {
    //         Some(d) => *d,
    //         None => 0
    //     };
    //     println!("{:?}", result);
    // }

    // {
    //     #[derive(Debug)]
    //     struct NumberCategory {
    //         even: Vec<i32>,
    //         odd: Vec<i32>,
    //     }
    //     let data_vec = vec![1, 2, 3, 4];
    //     let data_grouped = data_vec.iter().fold(
    //         NumberCategory{ even: Vec::new(), odd: Vec::new() }, 
    //         |mut group, each| {
    //             if *each % 2 == 0 {
    //                 group.even.push(*each)
    //             } else {
    //                 group.odd.push(*each)
    //             }
    //             group
    //         }
    //     );
    //     println!("{:?}", data_grouped);
    // }

    // {
    //     let data_vec = vec![1, 2, 3, 4];
    //     let result = data_vec.iter()
    //         .cloned()
    //         .inspect(|x| println!("about to filter: {x}"))
    //         .filter(|x| x % 2 == 0)
    //         .inspect(|x| println!("made it through filter: {x}"))
    //         .fold(0, |sum, i| sum + i);
    //     println!("sum: {:?}", result);
    // }

    // {
    //     let data_vec = vec![1.1, 2.2, 3.3, 4.5];
    //     let result: f64 = data_vec.iter().sum();
    //     println!("sum: {:?}", result);
    // }

    // {
    //     let data_vec = vec![1, 2, 3, 4];

    //     let max_number = data_vec.iter().reduce(|left, right| {
    //         print!("left ({left}) vs right ({right})");

    //         if *left >= *right { 
    //             println!(" -> left ({left}) is greater");
    //             left
    //         } else {
    //             println!(" -> right ({right}) is greater");
    //             right
    //         }
    //     });

    //     match max_number {
    //         Some(n) => println!("max_number: {:?}", n),
    //         None => println!("no data found"),
    //     }
    // }

    // {
    //     let data_vec = vec![1, 2, 3, 4];

    //     let min = data_vec.iter().min().unwrap();
    //     println!("min: {min}");

    //     let max = data_vec.iter().max().unwrap();
    //     println!("max: {max}");
    // }

    // {
    //     let mut data_vec = vec![2, 3, 1, 4];
    //     println!("before: {data_vec:?}");

    //     data_vec.sort();
    //     println!("after: {data_vec:?}");
    // }
}
