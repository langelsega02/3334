// Declare a constant for the freezing point of water in Fahrenheit
const FREEZING_POINT_F: f64 = 32.0;

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    // Declare a mutable variable with a temperature in Fahrenheit
    let mut temp_fahrenheit: f64 = 32.0;

    // Convert and print the temperature in Celsius
    let temp_celsius = fahrenheit_to_celsius(temp_fahrenheit);
    println!("{:.1}°F is {:.1}°C", temp_fahrenheit, temp_celsius);

    // Loop to convert and print the next 5 integer temperatures
    for i in 1..=5 {
        temp_fahrenheit += 1.0;
        let temp_celsius = fahrenheit_to_celsius(temp_fahrenheit);
        println!("{:.1}°F is {:.1}°C", temp_fahrenheit, temp_celsius);
    }
}
