use std::fs;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    //let content = fs::read_to_string(filename)?;
    let content = match fs::read_to_string(filename) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };
    let chars = content.chars().count();
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    
    println!("Characters: {}", chars);
    println!("Lines: {}", lines);
    println!("Words: {}", words);
    Ok(())
}