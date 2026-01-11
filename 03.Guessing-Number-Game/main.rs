use rand::Rng;
use std::io::{self, Write};

fn main() {
    let max_value = {
        let input = read_input("Enter max value (min 10): ").trim().to_string();

        match input.parse::<usize>() {
            Ok(len) if len >= 9 => len,
            _ => {
                println!("Invalid value: Using default max value of 100.");
                100
            }
        }
    };

    let secret_value = generate_rand_value(max_value);
    loop {
        let guess = read_input("Guess: ").trim().to_string();
        let check = compare_guess(guess, secret_value);
        
        if check == 0 {
            println!("Correct! You guessed it!");
            break;
        } else if check > 0 {
            println!("Too high!");
        } else if check < 0 {
            println!("Too low!");
        }
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    self::io::stdout().flush().unwrap();
    
    let mut input = String::new();
    self::io::stdin().read_line(&mut input).unwrap();
    input
}

fn compare_guess(guess: String, secret_value: usize) -> isize {
    match guess.parse::<usize>() {
        Ok(guess_num) => guess_num as isize - secret_value as isize,
        Err(_) => {
            println!("Invalid input!");
            -1
        }
    }
}

fn generate_rand_value(max_value: usize) -> usize {
    let mut rng = rand::thread_rng();
    let n: usize = rng.gen_range(0..max_value);
    n
}

