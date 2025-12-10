use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_format_json() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.json");

    // Write unformatted JSON
    let unformatted = r#"{"name":"Alice","age":30,"city":"New York"}"#;
    fs::write(&test_file, unformatted).unwrap();

    // Run jsonf
    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg(&test_file)
        .output()
        .unwrap();

    assert!(output.status.success());

    // Read formatted result
    let formatted = fs::read_to_string(&test_file).unwrap();

    // Verify it's properly formatted (keys are sorted alphabetically by serde_json)
    let expected = r#"{
  "age": 30,
  "city": "New York",
  "name": "Alice"
}
"#;
    assert_eq!(formatted, expected);
}

#[test]
fn test_format_nested_json() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("nested.json");

    // Write unformatted nested JSON
    let unformatted =
        r#"{"person":{"name":"Bob","details":{"age":25,"hobbies":["reading","coding"]}}}"#;
    fs::write(&test_file, unformatted).unwrap();

    // Run jsonf
    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg(&test_file)
        .output()
        .unwrap();

    assert!(output.status.success());

    // Read formatted result
    let formatted = fs::read_to_string(&test_file).unwrap();

    // Verify it's properly formatted (keys are sorted alphabetically by serde_json)
    let expected = r#"{
  "person": {
    "details": {
      "age": 25,
      "hobbies": [
        "reading",
        "coding"
      ]
    },
    "name": "Bob"
  }
}
"#;
    assert_eq!(formatted, expected);
}

#[test]
fn test_invalid_json() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("invalid.json");

    // Write invalid JSON
    fs::write(&test_file, "not valid json").unwrap();

    // Run jsonf
    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg(&test_file)
        .output()
        .unwrap();

    // Should fail
    assert!(!output.status.success());
}

#[test]
fn test_nonexistent_file() {
    // Run jsonf on a file that doesn't exist
    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg("/tmp/nonexistent_file_12345.json")
        .output()
        .unwrap();

    // Should fail
    assert!(!output.status.success());
}
