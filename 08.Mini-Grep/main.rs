use std::fs;
use std::env;

use std::io;
use std::io::Write;

fn main() -> std::io::Result<()>
{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
        eprintln!("Usage: {} example.txt", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let content = fs::read_to_string(filename)?;
    let grep = read_input("What to grep: \n").trim().to_string();
    println!("Looking for '{}':", grep);
    
    let mut count = 0;
    for line in content.lines()
    {
        if line.contains(&grep) 
        {
            count += 1;
            println!("{} -> {}", count, line);
            println!();
        }
    }
    println!("Found {} matches", count);
    Ok(())
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
