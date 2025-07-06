use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: cargo run <num1> \"<operator>\" <num2>");
        return;
    }
    let num1 = &args[1];
    match num1.parse::<f64>() {
        Ok(_) => {}
        Err(_) => {
            println!("Error: '{}' is not a valid number.", num1);
            return;
        }
    }
    let operator = &args[2];
    if operator.len() != 1 {
        println!("Error: Operator must be a single character.");
        return;
    }
    let num2 = &args[3];
    match num2.parse::<f64>() {
        Ok(_) => {}
        Err(_) => {
            println!("Error: '{}' is not a valid number.", num2);
            return;
        }
    }
    let result: f64;
    let operand1 = num1.parse::<f64>().unwrap();
    let operand2 = num2.parse::<f64>().unwrap();
    match operator.as_str() {
        "+" => result = operand1 + operand2,
        "-" => result = operand1 - operand2,
        "*" => result = operand1 * operand2,
        "/" => {
            if operand2 == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            result = operand1 / operand2;
        }
        _ => {
            println!("Error: Unsupported operator '{}'.", operator);
            return;
        }
    }
    println!("{num1} {operator} {num2} = {result}");
}

