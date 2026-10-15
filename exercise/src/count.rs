fn main() {
    let x = 5; // Immutable by default
    println!("The value of x is: {}", x);
    
    // This would cause a compile error:
    // x = 6; // Cannot assign twice to immutable variable
    
    // To make a variable mutable, use 'mut':
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6; // This works!
    println!("The value of y is now: {}", y);
}
