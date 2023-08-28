use std::io;

fn main() {
    println!("Enter the temperatue in Fahrenheit:");

    let mut fahrenheit_temp = String::new();

    io::stdin()
        .read_line(&mut fahrenheit_temp)
        .expect("Encountered an error!");

    let fahrenheit: f64 = fahrenheit_temp.trim().parse().expect("Please enter a valid number.");

    let celsius = (fahrenheit - 32.0) * 5.0/9.0;

    println!("{}F is equal to {}C", fahrenheit, celsius);     
    
}
