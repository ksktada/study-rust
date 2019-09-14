fn main() {
    print_number(5);
    
    another_function(5, 6);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
