use std::io;

fn temp_conv() {
    println!("Insert a temperature");

    let mut temp = String::new();
    let mut scale = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    println!("Was it Fahrenheit (f) or Celsius (c)?");

    io::stdin()
        .read_line(&mut scale)
        .expect("Failed to read line");

    if scale.trim() == "f" {
        let temp_fahrenheit: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => return,
        };

        let temp_celsius = (temp_fahrenheit - 32.0) / 1.8;

        println!("Temperature in °C: {temp_celsius}");

        return;
    } else if scale.trim() == "c" {
        let temp_celsius: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => return,
        };

        let temp_fahrenheit = temp_celsius * 1.8 + 32.0;

        println!("Temperature in °F: {temp_fahrenheit}");

        return;
    } else {
        println!("You did not enter 'f' or 'c'");
        return;
    }
}
