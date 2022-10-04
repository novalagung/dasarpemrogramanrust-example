fn main() {
    // let numbers = [12, 16, 8, 3];
    // println!("numbers   : {:?}, len: {}", numbers, numbers.len());
    // println!("numbers[0]: {:?}", numbers[0]);
    // println!("numbers[1]: {:?}", numbers[1]);

    // let slice_a = &numbers[0..3];
    // println!("slice_a   : {:?}, len: {}", slice_a, slice_a.len());
    // println!("slice_a[0]: {:?}", slice_a[0]);
    // println!("slice_a[1]: {:?}", slice_a[1]);

    // let slice_b = &slice_a[1..=2];
    // println!("slice_b   : {:?}, len: {}", slice_b, slice_b.len());
    // println!("slice_b[0]: {:?}", slice_b[0]);
    // println!("slice_b[1]: {:?}", slice_b[1]);

    // ==============================

    // let mut numbers2 = [12, 16, 8, 3];
    // println!("===== before =====");
    // println!("numbers2 : {:?} \n", numbers2);

    // let slice_e = &mut numbers2[..=2];
    // slice_e[1] = 99;

    // println!("===== after =====");
    // println!("slice_e  : {:?}", slice_e);
    // println!("numbers2 : {:?}", numbers2);

    // ==============================

    // let scores1 = [7, 8, 9];

    // for score in &scores1[..] {
    //     print!("{:?} ", score);
    // }

    // ==============================

    let mut scores2 = [7, 8, 9];
    println!("(before) scores2 : {:?}", scores2);

    let slice_f = &mut scores2[..];

    for score in &mut slice_f[..] {
        *score += 1;
    }

    println!("(after)  scores2 : {:?}", scores2);

}
