use std::fs;
use std::env;

use serde_json::{Value};


fn flatten_json(value: &Value, prefix: String, result: &mut Vec<(String, String)>) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                flatten_json(val, new_prefix, result);
            }
        }
        Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                let new_prefix = format!("{}[{}]", prefix, i);
                flatten_json(val, new_prefix, result);
            }
        }
        Value::String(s) => {
            result.push((prefix, format!("\"{}\"", s)));
        }
        Value::Number(n) => {
            result.push((prefix, n.to_string()));
        }
        Value::Bool(b) => {
            result.push((prefix, b.to_string()));
        }
        Value::Null => {
            result.push((prefix, "null".to_string()));
        }
    }
}

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

    let mut flattened = Vec::new();
    flatten_json(&v, String::new(), &mut flattened);
    
    for (path, value) in flattened {
        println!("{} = {}", path, value);
    }

    Ok(())
}