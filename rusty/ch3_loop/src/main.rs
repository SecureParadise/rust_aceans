use std::io;

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_line(&mut input_data)
        .expect("Failed to read line");

    let values: Vec<&str> = input_data.trim().split_whitespace().collect();

    // Make sure we have two inputs
    if values.len() < 2 {
        println!("Please enter two numbers: Celsius and Fahrenheit values.");
        return;
    }

    let celsius_input: f32 = values[0].parse().expect("Invalid Celsius value");
    let fahrenheit_input: f32 = values[1].parse().expect("Invalid Fahrenheit value");

    let fahrenheit_result = celsius_fahrenheit(celsius_input);
    let celsius_result = fahrenheit_celsius(fahrenheit_input);

    println!("{celsius_input}°C = {fahrenheit_result}°F");
    println!("{fahrenheit_input}°F = {celsius_result}°C");
}

fn celsius_fahrenheit(deg_celsius: f32) -> f32 {
    (deg_celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_celsius(deg_fahrenheit: f32) -> f32 {
    (deg_fahrenheit - 32.0) * 5.0 / 9.0
}

/*
let mut input = String::new();

    println!("Enter a float:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num: f32 = input.trim().parse::<f32>().expect("Please enter a valid float");

*/