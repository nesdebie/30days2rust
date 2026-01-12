use std::fs;
use std::env;

use serde_json::{Value};

fn main() -> std::io::Result<()>
{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
        eprintln!("Usage: {} example.json", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let raw = fs::read_to_string(filename)?;
    let v: Value = serde_json::from_str(&raw)?;

    eprintln!("{}", raw);
    eprintln!("{}", v);

    Ok(())
}