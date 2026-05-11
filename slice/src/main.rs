fn main() {
    // A.14.1 - slice basics
    let numbers = [12, 16, 8, 3];
    // variabel numbers isinya array [12, 16, 8, 3]

    println!("numbers   : {:?}, len: {}", numbers, numbers.len());
    println!("numbers[0]: {:?}", numbers[0]);
    println!("numbers[1]: {:?}", numbers[1]);

    // meminjam data milik numbers elemen ke-0 hingga sebelum 3 (yaitu 2)
    // hasilnya adalah [12, 16, 8]
    let slice_a = &numbers[0..3];
    println!("slice_a   : {:?}, len: {}", slice_a, slice_a.len());
    println!("slice_a[0]: {:?}", slice_a[0]);
    println!("slice_a[1]: {:?}", slice_a[1]);

    // meminjam data milik slice_a elemen ke-1 hingga 2
    // hasilnya adalah [16, 8]
    let slice_b = &slice_a[1..=2];
    println!("slice_b   : {:?}, len: {}", slice_b, slice_b.len());
    println!("slice_b[0]: {:?}", slice_b[0]);
    println!("slice_b[1]: {:?}", slice_b[1]);

    // A.14.2 - size slice
    println!("{}", numbers.len());
    println!("{}", slice_a.len());
    println!("{}", slice_b.len());

    // A.14.3 - range syntax
    let data = ["a", "b", "c", "d"];

    let sliced_data = &data[1..3];
    println!("{:?}", sliced_data);

    let sliced_data = &data[1..=3];
    println!("{:?}", sliced_data);

    let sliced_data = &data[..3];
    println!("{:?}", sliced_data);

    let sliced_data = &data[..=2];
    println!("{:?}", sliced_data);

    let sliced_data = &data[1..];
    println!("{:?}", sliced_data);

    let sliced_data = &data[..];
    println!("{:?}", sliced_data);

    // A.14.4 - mutability pada slice
    let mut numbers2 = [12, 16, 8, 3];
    println!("===== before =====");
    println!("numbers2 : {:?}", numbers2);

    let slice_e = &mut numbers2[..=2];
    slice_e[1] = 99;

    println!("===== after =====");
    println!("slice_e  : {:?}", slice_e);
    println!("numbers2 : {:?}", numbers2);

    // A.14.5 - for in pada slice
    let scores1 = [7, 8, 9];

    for score in &scores1[..] {
        print!("{:?} ", score);
    }
    println!();

    // A.14.6 - for in pada mutable slice
    let mut scores2 = [7, 8, 9];
    println!("(before) scores2 : {:?}", scores2);

    let slice_f = &mut scores2[..];

    for score in &mut slice_f[..] {
        *score += 1;
    }

    println!("(after)  scores2 : {:?}", scores2);
}
