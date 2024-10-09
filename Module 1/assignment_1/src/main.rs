// Define an Immutable constant for the freezing point of water, 32F
const FREEZING_POINT: f64 = 32.0;

// Conversion functions:
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (5.0 * (f - FREEZING_POINT)) / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    ((9.0* c) / 5.0) + FREEZING_POINT
}

fn main() {

    // Define a Mutable variable for some temperature xxF
    let mut temperature = 16.0;

    celsius_to_fahrenheit(temperature);

    // Convert to celcius and print result
    print!("Original temperature: {}, ", temperature);
    
    temperature = fahrenheit_to_celsius(temperature);
    
    println!("Conversion to Celsius: {}", temperature);

    // Use a loop to convert the next 5 integers
    for offset in 1..6 {

        temperature = 16.0 + offset as f64;

        print!("New temperature: {}, ", temperature);

        temperature = fahrenheit_to_celsius(temperature);

        println!("Conversion to Celsius: {}", temperature);
    }
}