fn main() {
    let x = 144.03;
    let y = 146.02;

    let z = x + y;

    println!("Addition: {}", z);

    println!("Conversion i32: {}", z as u32);
    println!("Conversion u8: {}", z as u8);

    let big: i32 = 300;
    let as_byte = big as u8; // A

    let a = 300;
    let b = a.to_string();

    let c = "256.45";
    let d = c.parse::<f64>().unwrap();

    println!("{} {} {}", as_byte, b, d);
}
