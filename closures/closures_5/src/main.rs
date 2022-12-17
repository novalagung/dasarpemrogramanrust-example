fn main() {
    {
        run_x_times(4, |i: i32| println!("hello rust {i}"));
    }

    {
        let numbers = [24, 13, 2, 53, 3];
        let number_to_find = 53;
        let index = find_index(&numbers, |e: &i32| -> bool {
            if *e == number_to_find {
               true 
            } else {
                false
            }
        });
    
        println!("number_to_find: {number_to_find}");
        println!("index: {index}");
    }
}

fn run_x_times<F>(x: i32, my_closure: F)
where
    F: Fn(i32),
{
    for i in 0..x {
        my_closure(i)
    }
}

fn find_index<T, F>(data: &[T], cond_fn: F) -> i32
where
    F: Fn(&T) -> bool,
{
    for i in 0..data.len() {
        if cond_fn(&data[i]) {
            return i as i32
        }
    }

    return -1
}
