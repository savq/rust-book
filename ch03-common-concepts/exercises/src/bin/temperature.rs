// Rustbook ch. 03, exercise 1: Convert temperatures between Fahrenheit and Celsius.
use std::io;

fn main() {
    loop {
        println!("\nTemperature converter\n1. °F to °C\n2. °C to °F");
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line"); // Handle Result

        // Shadow option
        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("select a valid option");
                continue;
            }
        };
        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line"); // Handle Result

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Write a valid number");
                continue;
            }
        };

        if option == 1 {
            println!(
                "{}°C is {}°F",
                temperature,
                fahrenheit_to_celsius(temperature)
            );
        } else if option == 2 {
            println!(
                "{}°C is {}°F",
                temperature,
                celsius_to_fahrenheit(temperature)
            );
        }
    }
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    c * 9.0 / 5.0 + 32.0
}
