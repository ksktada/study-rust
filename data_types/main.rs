fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
