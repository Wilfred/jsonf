use clap::Parser;
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "jsonf")]
#[command(version)]
#[command(about = "A simple JSON formatter that pretty-prints JSON files", long_about = None)]
struct Cli {
    /// Paths to JSON files to format
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Sort arrays in addition to formatting
    #[arg(short, long)]
    sort: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut had_error = false;

    for file in &cli.files {
        if let Err(err) = format_file(file, cli.sort) {
            eprintln!("{err}");
            had_error = true;
        }
    }

    if had_error {
        process::exit(1);
    }
}

fn format_file(file: &PathBuf, sort: bool) -> Result<(), String> {
    // Read the file
    let content = fs::read_to_string(file)
        .map_err(|err| format!("Error reading file '{}': {}", file.display(), err))?;

    // Parse JSON
    let mut json_value: serde_json::Value = serde_json::from_str(&content).map_err(|err| {
        let mut msg = format!("Error parsing JSON in '{}': {}", file.display(), err);

        // Show the problematic line with line number
        let line_num = err.line();
        let col_num = err.column();
        if let Some(line) = content.lines().nth(line_num - 1) {
            msg.push_str(&format!("\n\nAt line {}:", line_num));
            msg.push_str(&format!("\n  {}", line));

            // Show a caret pointing to the error column
            if col_num > 0 {
                msg.push_str(&format!("\n  {}^", " ".repeat(col_num - 1)));
            }
        }

        msg
    })?;

    // Drop the original content string to free memory before generating output
    drop(content);

    if sort {
        sort_arrays(&mut json_value);
    }

    // Format JSON with pretty printing and write using a buffered writer
    let out_file = fs::File::create(file)
        .map_err(|err| format!("Error writing file '{}': {}", file.display(), err))?;
    let mut writer = std::io::BufWriter::new(out_file);
    serde_json::to_writer_pretty(&mut writer, &json_value)
        .map_err(|err| format!("Error formatting JSON: {}", err))?;
    writer
        .write_all(b"\n")
        .map_err(|err| format!("Error writing file '{}': {}", file.display(), err))?;

    println!("Formatted {}", file.display());
    Ok(())
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
            // Compare structurally by iterating keys, avoiding expensive serialization
            let mut a_keys: Vec<&String> = a.keys().collect();
            let mut b_keys: Vec<&String> = b.keys().collect();
            a_keys.sort();
            b_keys.sort();
            match a_keys.cmp(&b_keys) {
                std::cmp::Ordering::Equal => {}
                ord => return ord,
            }
            for key in a_keys {
                let ord = compare_values(&a[key], &b[key]);
                if ord != std::cmp::Ordering::Equal {
                    return ord;
                }
            }
            std::cmp::Ordering::Equal
        }
    }
}
