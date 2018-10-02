fn main() {
    let mut x = 5;
    println!("X = {}", x);
    x = 6;
    println!("X = {}", x);
    //shadowing with immutable variables
    let y = 5;
    let y = y + 7;
    let y = y * 2;
    println!("Y = {}", y);
    //tuple example
    let tuple: (i32, u8, f64) = (23, b'a', 5.98);
    let (a, b, c) = tuple;
    println!("A={}, B={}, C={}", a, b, c);
    //array example
    let arr: [i32; 4] = [2, 4, 6, 8];
    println!("First element: {}", arr[0]);
    println!("Third element: {}", arr[2]);
}
