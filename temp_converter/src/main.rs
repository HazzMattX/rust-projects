use std::io::Write;
use TemperatureScale::*;
enum TemperatureScale {
    CelsiusToFahrenheit,
    CelsiusToKelvin,
    FahrenheitToCelsius,
    FahrenheitToKelvin,
    KelvinToCelsius,
    KelvinToFahrenheit,
}
fn convert_temperature(temp: f64, unit: TemperatureScale) -> f64 {
    match unit {
        CelsiusToFahrenheit => (temp * 9.0 / 5.0) + 32.0,
        CelsiusToKelvin => temp + 273.15,
        FahrenheitToCelsius => (temp - 32.0) * 5.0 / 9.0,
        FahrenheitToKelvin => (temp - 32.0) * 5.0 / 9.0 + 273.15,
        KelvinToCelsius => temp - 273.15,
        KelvinToFahrenheit => (temp - 273.15) * 9.0 / 5.0 + 32.0,
    }
}
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
fn main() {
    let number: f64 = get_input("Please enter a number\n")
    .trim().parse().unwrap_or(0.0);
let scale;
// Chooses conversion
use TemperatureScale::*;
let convert = get_input("Choose temperature unit\n");
let convert: TemperatureScale = match convert.trim() {
    "1" => { scale = "Fahrenheit".to_string(); CelsiusToFahrenheit },
    "2" => { scale = "Kelvin".to_string(); CelsiusToKelvin },
    "3" => { scale = "Celsius".to_string(); FahrenheitToCelsius },
    "4" => { scale = "Kelvin".to_string(); FahrenheitToKelvin },
    "5" => { scale = "Celsius".to_string(); KelvinToCelsius },
    "6" => { scale = "Fahrenheit".to_string(); KelvinToFahrenheit },
    _ => {
        println!("Invalid input, defaulting to Celsius to Fahrenheit.");
        scale = "Fahrenheit".to_string();
        CelsiusToFahrenheit
    }
};
    let result = convert_temperature(number, convert);
    println!("It is {} degrees {}", result, scale);
}