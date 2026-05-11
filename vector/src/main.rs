use std::collections::VecDeque;

fn main() {
    // A.16.1 - deklarasi vektor
    let mut data_one = vec!["batman", "superman", "lobo"];
    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(), data_one.capacity());

    // A.16.1 - pop
    data_one.pop();
    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(), data_one.capacity());

    // A.16.1 - remove
    data_one.remove(1);
    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(), data_one.capacity());

    // A.16.1 - push
    data_one.push("constantine");
    data_one.push("trigon");
    data_one.push("darkseid");
    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(), data_one.capacity());

    // A.16.1 - modify element
    data_one[2] = "red hood";
    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(), data_one.capacity());

    // A.16.1 - is_empty
    let is_vector_empty = data_one.is_empty();
    println!("result: {:?}", is_vector_empty);

    // A.16.1 - clear
    data_one.clear();
    println!("data: {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(), data_one.capacity());

    // A.16.1 - append
    let mut result_one = vec![3, 1, 2];
    let mut data_two = vec![7, 6, 8];
    result_one.append(&mut data_two);
    println!("data: {:?}", result_one);
    println!("length: {}, capacity: {}", result_one.len(), result_one.capacity());

    // A.16.1 - append inline
    result_one.append(&mut vec![4, 5]);
    println!("data: {:?}", result_one);
    println!("length: {}, capacity: {}", result_one.len(), result_one.capacity());

    // A.16.1 - sort
    println!("data: {:?}", result_one);
    result_one.sort();
    println!("data: {:?}", result_one);

    // A.16.2 - macam deklarasi vector
    let mut vector_4 = vec![1, 2, 3];
    let mut vector_5: Vec<i64> = vec![1, 2, 3];
    println!("{vector_4:?} {vector_5:?}");

    let vector_7: Vec<&str> = vec![];
    let vector_8: Vec<&str> = Vec::new();
    println!("{vector_7:?} {vector_8:?}");

    // A.16.3 - for in
    let vec_eight = vec![1, 2, 3];
    for e in vec_eight {
        print!("{e} ");
    }
    println!();

    let vec_nine = vec![1, 2, 3];
    for i in 0..vec_nine.len() {
        print!("{} ", vec_nine[i]);
    }
    println!();

    // A.16.4 - ownership (error version commented)
    // let vec_ten = vec![1, 2, 3];
    // for e in vec_ten {
    //     print!("{e} ");
    // }
    // for i in 0..vec_ten.len() {
    //     print!("{} ", vec_ten[i]);
    // }

    // A.16.4 - fix with borrowing
    let vec_ten = vec![1, 2, 3];
    for e in &vec_ten {
        print!("{e} ");
    }
    for i in 0..vec_ten.len() {
        print!("{} ", vec_ten[i]);
    }
    println!();

    // A.16.4 - fix with iter
    let vec_ten = vec![1, 2, 3];
    for e in vec_ten.iter() {
        print!("{e} ");
    }
    println!();

    // A.16.5 - vector slice
    let vec_population = vec![2, 1, 3];
    let vec_sample = &vec_population[0..1];
    println!("{:?}", vec_sample);

    // A.16.6 - VecDeque
    let mut vec_10 = VecDeque::from(vec!["a", "b", "c"]);

    vec_10.pop_front();
    vec_10.push_front("z");
    println!("data: {:?}", vec_10);

    vec_10.pop_back();
    vec_10.push_back("h");
    println!("data: {:?}", vec_10);
}
