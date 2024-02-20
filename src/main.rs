use boson_rust::linalg::{Vec4, ID_MAT4X4};
fn main() {
    println!("Hello, world!");
    let v = Vec4 {
        data: [1.0, 2.0, 3.0, 1.0],
    };
    let v_new: Vec4 = ID_MAT4X4 / 4.0 * v;
    println!("{}", v_new.data[0]);
    println!("{}", v_new.data[1]);
    println!("{}", v_new.data[2]);
    println!("{}", v_new.data[3]);
}
