mod unit_converter;
use unit_converter::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        print_usage();
        return;
    }
    
    let conversion_type = &args[1];
    let value: f64 = match args[2].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: Invalid number '{}'", args[2]);
            print_usage();
            return;
        }
    };
    
    let result = match conversion_type.as_str() {
        "km2m" => km2m(value),
        "m2km" => m2km(value),
        "miles2km" => miles2km(value),
        "km2miles" => km2miles(value),
        "c2f" => celsius2fahrenheit(value),
        "f2c" => fahrenheit2celsius(value),
        "kg2lbs" => kg2lbs(value),
        "lbs2kg" => lbs2kg(value),
        _ => {
            eprintln!("Error: Unknown conversion type '{}'", conversion_type);
            print_usage();
            return;
        }
    };
    
    println!("{}", result);
}

fn print_usage() {
    println!("Usage: unit_converter <conversion_type> <value>");
    println!("\nAvailable conversions:");
    println!("  km2m      - Kilometers to meters");
    println!("  m2km      - Meters to kilometers");
    println!("  miles2km  - Miles to kilometers");
    println!("  km2miles  - Kilometers to miles");
    println!("  c2f       - Celsius to Fahrenheit");
    println!("  f2c       - Fahrenheit to Celsius");
    println!("  kg2lbs    - Kilograms to pounds");
    println!("  lbs2kg    - Pounds to kilograms");
    println!("\nExample: ./unit_converter km2m 5");
}