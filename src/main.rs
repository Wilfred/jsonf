use clap::Parser;
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
    let json_value: serde_json::Value = match serde_json::from_str(&content) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            process::exit(1);
        }
    };

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
