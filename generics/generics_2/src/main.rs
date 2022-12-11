struct Point<T, U> {
    x: T,
    y: T,
    z: U
}

impl<T, U> Point<T, U> {

    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }

    fn get_z(&self) -> &U {
        &self.z
    }
}


// impl Point<i32, f64> {

//     fn get_x(&self) -> &i32 {
//         &self.x
//     }

//     fn get_y(&self) -> &i32 {
//         &self.y
//     }

//     fn get_z(&self) -> &f64 {
//         &self.z
//     }
// }

enum Kendaraan<T> {
    Skateboard,
    SepedaPancal,
    Gledekan(T),
}

fn main() {
    let num_one: Point<i32, f64> = Point { x: 502, y: 120, z: 4.5 };
    println!("{} {} {}", num_one.get_x(), num_one.get_y(), num_one.get_z());
    // 502 120 4.5

    let num_two: Point<f64, i32> = Point { x: 1.2, y: 4.3, z: 534 };
    println!("{} {} {}", num_two.get_x(), num_two.get_y(), num_two.get_z());
    // 1.2 4.3 534

    let kendaraan1 = Kendaraan::<&str>::Skateboard;
    let kendaraan2 = Kendaraan::<&str>::SepedaPancal;
    let kendaraan3 = Kendaraan::<&str>::Gledekan("Artco");
}
