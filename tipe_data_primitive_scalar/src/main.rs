fn main() {
    let numerik1 = 24;
    let numerik2: i8 = 127;
    let numerik3: i64 = 12;
    println!("{} | {} | {}", numerik1, numerik2, numerik3);
    
    let numerik4: u32 = 28;
    let numerik5: u8 = 16;
    let numerik6: u64 = 42;
    println!("{} | {} | {}", numerik4, numerik5, numerik6);
    
    let fp1: f32 = 3.14;
    let fp2: f64 = 3.1415926535;
    println!("{} | {:.5}", fp1, fp2);

    let b1 = true;
    let b2 = false;
    println!("{} | {}", b1, b2);

    let c1 = 'n';
    let c2 = '-';
    let c3 = '2';
    println!("{} | {} | {}", c1, c2, c3);

    let ptr1: &i32 = &123;
    println!("{}", ptr1);
}
