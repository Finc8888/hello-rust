use std::io;

fn main() {
    let choice = is_fahrenheit_to_celsius();

    if choice {
        // Convert Fahrenheit to Celsius
        println!("You chose to convert from Fahrenheit to Celsius.");
        println!("Enter temperature in Fahrenheit:");

        let temperature = get_temperature_input();
        let converted = f_to_c(temperature);

        display_conversion_result(temperature, converted, "°F", "°C");
    } else {
        // Convert Celsius to Fahrenheit
        println!("You chose to convert from Celsius to Fahrenheit.");
        println!("Enter temperature in Celsius:");

        let temperature = get_temperature_input();
        let converted = c_to_f(temperature);

        display_conversion_result(temperature, converted, "°C", "°F");
    }
}

// Main function for selecting the conversion direction
// returns true if direct conversion from Fahrenheit to Celsius is selected
// returns false if reverse conversion from Celsius to Fahrenheit is selected
fn is_fahrenheit_to_celsius() -> bool {
    loop {
        println!("Temperature Converter");
        println!("1. Convert Fahrenheit to Celsius");
        println!("2. Convert Celsius to Fahrenheit");
        println!("Please enter your choice (1 or 2):");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim().parse::<i32>() {
            Ok(1) => return true,
            Ok(2) => return false,
            Ok(_) => println!("Please enter only 1 or 2"),
            Err(_) => println!("Please enter a valid number"),
        }
    }
}

// Function to get temperature input from user
fn get_temperature_input() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(temp) => return temp,
            Err(_) => println!("Please enter a valid number:"),
        }
    }
}

// Function to display conversion result
fn display_conversion_result(original: f64, converted: f64, from_unit: &str, to_unit: &str) {
    println!("{:.2}{} = {:.2}{}", original, from_unit, converted, to_unit);
}

// Convert from Fahrenheit to Celsius: C = (F - 32) × 5/9
fn f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

// Convert from Celsius to Fahrenheit: F = C × 9/5 + 32
fn c_to_f(celsius: f64) -> f64 {
    celsius * (9.0 / 5.0) + 32.0
}
