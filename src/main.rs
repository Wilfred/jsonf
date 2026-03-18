use clap::Parser;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "jsonf")]
#[command(version)]
#[command(about = "A simple JSON formatter that pretty-prints JSON files", long_about = None)]
struct Cli {
    /// Path to the JSON file to format
    file: PathBuf,

    /// Sort arrays in addition to formatting
    #[arg(short, long)]
    sort: bool,
}

fn main() {
    let cli = Cli::parse();

    // Read the file
    let content = match fs::read_to_string(&cli.file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", cli.file.display(), err);
            process::exit(1);
        }
    };

    // Parse JSON
    let mut json_value: serde_json::Value = match serde_json::from_str(&content) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);

            // Show the problematic line with line number
            let line_num = err.line();
            let col_num = err.column();
            let lines: Vec<&str> = content.lines().collect();

            if line_num > 0 && line_num <= lines.len() {
                eprintln!("\nAt line {}:", line_num);
                eprintln!("  {}", lines[line_num - 1]);

                // Show a caret pointing to the error column
                if col_num > 0 {
                    eprintln!("  {}^", " ".repeat(col_num - 1));
                }
            }

            process::exit(1);
        }
    };

    if cli.sort {
        sort_arrays(&mut json_value);
    }

    // Format JSON with pretty printing
    let formatted = match serde_json::to_string_pretty(&json_value) {
        Ok(formatted) => formatted,
        Err(err) => {
            eprintln!("Error formatting JSON: {}", err);
            process::exit(1);
        }
    };

    // Write back to the file
    if let Err(err) = fs::write(&cli.file, formatted + "\n") {
        eprintln!("Error writing file '{}': {}", cli.file.display(), err);
        process::exit(1);
    }

    println!("Formatted {}", cli.file.display());
}

fn sort_arrays(value: &mut Value) {
    match value {
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                sort_arrays(item);
            }
            arr.sort_by(compare_values);
        }
        Value::Object(map) => {
            for (_key, val) in map.iter_mut() {
                sort_arrays(val);
            }
        }
        _ => {}
    }
}

fn compare_values(a: &Value, b: &Value) -> std::cmp::Ordering {
    match (a, b) {
        (Value::Null, Value::Null) => std::cmp::Ordering::Equal,
        (Value::Null, _) => std::cmp::Ordering::Less,
        (_, Value::Null) => std::cmp::Ordering::Greater,
        (Value::Bool(a), Value::Bool(b)) => a.cmp(b),
        (Value::Bool(_), _) => std::cmp::Ordering::Less,
        (_, Value::Bool(_)) => std::cmp::Ordering::Greater,
        (Value::Number(a), Value::Number(b)) => {
            let a = a.as_f64().unwrap_or(0.0);
            let b = b.as_f64().unwrap_or(0.0);
            a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
        }
        (Value::Number(_), _) => std::cmp::Ordering::Less,
        (_, Value::Number(_)) => std::cmp::Ordering::Greater,
        (Value::String(a), Value::String(b)) => a.cmp(b),
        (Value::String(_), _) => std::cmp::Ordering::Less,
        (_, Value::String(_)) => std::cmp::Ordering::Greater,
        (Value::Array(a), Value::Array(b)) => {
            for (ai, bi) in a.iter().zip(b.iter()) {
                let ord = compare_values(ai, bi);
                if ord != std::cmp::Ordering::Equal {
                    return ord;
                }
            }
            a.len().cmp(&b.len())
        }
        (Value::Array(_), _) => std::cmp::Ordering::Less,
        (_, Value::Array(_)) => std::cmp::Ordering::Greater,
        (Value::Object(a), Value::Object(b)) => {
            let a_str = serde_json::to_string(a).unwrap_or_default();
            let b_str = serde_json::to_string(b).unwrap_or_default();
            a_str.cmp(&b_str)
        }
    }
}
