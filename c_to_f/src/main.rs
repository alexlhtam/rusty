use std::io;

fn main() {
    println!("Input your temperature in Celsius:");

    let mut temp_c = String::new(); 

    io::stdin()
        .read_line(&mut temp_c)
        .expect("Failed to read line");

    let temp_c: f32 = temp_c.trim().parse().expect("Please input a float!");

    println!("You inputted: {temp_c} Celsius");
    
    let temp_f = temp_c * 1.8 + 32.0;

    println!("Your temperature in Fahrenheit: {temp_f} F");
}
