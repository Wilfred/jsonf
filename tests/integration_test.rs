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

    // Error message should contain line information
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("At line"));
}

#[test]
fn test_syntax_error_details() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("syntax_error.json");

    // Write JSON with a syntax error on a specific line
    let invalid_json = r#"{
  "name": "Alice",
  "age": 30,
  "city": "New York"
  "country": "USA"
}
"#;
    fs::write(&test_file, invalid_json).unwrap();

    // Run jsonf
    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg(&test_file)
        .output()
        .unwrap();

    // Should fail
    assert!(!output.status.success());

    // Error message should contain the problematic line and line number
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("At line"));
    assert!(stderr.contains("\"country\"") || stderr.contains("\"city\""));
}

#[test]
fn test_sort_flag_sorts_string_array() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("sort.json");

    let unformatted = r#"{"fruits":["banana","apple","cherry"]}"#;
    fs::write(&test_file, unformatted).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg("--sort")
        .arg(&test_file)
        .output()
        .unwrap();

    assert!(output.status.success());

    let formatted = fs::read_to_string(&test_file).unwrap();
    let expected = r#"{
  "fruits": [
    "apple",
    "banana",
    "cherry"
  ]
}
"#;
    assert_eq!(formatted, expected);
}

#[test]
fn test_sort_flag_sorts_number_array() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("sort_numbers.json");

    let unformatted = r#"{"values":[3,1,2]}"#;
    fs::write(&test_file, unformatted).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg("--sort")
        .arg(&test_file)
        .output()
        .unwrap();

    assert!(output.status.success());

    let formatted = fs::read_to_string(&test_file).unwrap();
    let expected = r#"{
  "values": [
    1,
    2,
    3
  ]
}
"#;
    assert_eq!(formatted, expected);
}

#[test]
fn test_sort_flag_sorts_nested_arrays() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("sort_nested.json");

    let unformatted = r#"{"data":{"tags":["zebra","alpha"],"nums":[5,2,8]}}"#;
    fs::write(&test_file, unformatted).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg("-s")
        .arg(&test_file)
        .output()
        .unwrap();

    assert!(output.status.success());

    let formatted = fs::read_to_string(&test_file).unwrap();
    let expected = r#"{
  "data": {
    "nums": [
      2,
      5,
      8
    ],
    "tags": [
      "alpha",
      "zebra"
    ]
  }
}
"#;
    assert_eq!(formatted, expected);
}

#[test]
fn test_without_sort_preserves_array_order() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("no_sort.json");

    let unformatted = r#"{"items":["banana","apple","cherry"]}"#;
    fs::write(&test_file, unformatted).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg(&test_file)
        .output()
        .unwrap();

    assert!(output.status.success());

    let formatted = fs::read_to_string(&test_file).unwrap();
    let expected = r#"{
  "items": [
    "banana",
    "apple",
    "cherry"
  ]
}
"#;
    assert_eq!(formatted, expected);
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

#[test]
fn test_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    let file1 = temp_dir.path().join("a.json");
    let file2 = temp_dir.path().join("b.json");

    fs::write(&file1, r#"{"key":"value1"}"#).unwrap();
    fs::write(&file2, r#"{"key":"value2"}"#).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg(&file1)
        .arg(&file2)
        .output()
        .unwrap();

    assert!(output.status.success());

    let formatted1 = fs::read_to_string(&file1).unwrap();
    assert_eq!(formatted1, "{\n  \"key\": \"value1\"\n}\n");

    let formatted2 = fs::read_to_string(&file2).unwrap();
    assert_eq!(formatted2, "{\n  \"key\": \"value2\"\n}\n");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Formatted"));
}

#[test]
fn test_format_jsonl() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.jsonl");

    // Write a JSONL file with records on each line. Some have extra whitespace
    // that should be normalized to compact form.
    let unformatted = "{\"name\":\"Alice\",\"age\":30}\n{  \"name\" : \"Bob\" , \"age\" : 25 }\n{\"name\":\"Carol\",\"age\":40}\n";
    fs::write(&test_file, unformatted).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg(&test_file)
        .output()
        .unwrap();

    assert!(output.status.success());

    let formatted = fs::read_to_string(&test_file).unwrap();
    // Each record is compacted to a single line. Keys are sorted alphabetically
    // by serde_json, and original record order is preserved without --sort.
    let expected = "{\"age\":30,\"name\":\"Alice\"}\n{\"age\":25,\"name\":\"Bob\"}\n{\"age\":40,\"name\":\"Carol\"}\n";
    assert_eq!(formatted, expected);
}

#[test]
fn test_format_jsonl_skips_blank_lines() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("blank.jsonl");

    let unformatted = "{\"a\":1}\n\n{\"b\":2}\n";
    fs::write(&test_file, unformatted).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg(&test_file)
        .output()
        .unwrap();

    assert!(output.status.success());

    let formatted = fs::read_to_string(&test_file).unwrap();
    let expected = "{\"a\":1}\n{\"b\":2}\n";
    assert_eq!(formatted, expected);
}

#[test]
fn test_sort_jsonl_sorts_lines_and_inner_arrays() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("sort.jsonl");

    // Records are out of order, and each record has an unsorted array.
    let unformatted = "{\"id\":3,\"tags\":[\"c\",\"a\",\"b\"]}\n{\"id\":1,\"tags\":[\"z\",\"x\"]}\n{\"id\":2,\"tags\":[\"m\",\"k\"]}\n";
    fs::write(&test_file, unformatted).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg("--sort")
        .arg(&test_file)
        .output()
        .unwrap();

    assert!(output.status.success());

    let formatted = fs::read_to_string(&test_file).unwrap();
    // Lines are sorted by JSON content (id ascending) and inner arrays are sorted.
    let expected = "{\"id\":1,\"tags\":[\"x\",\"z\"]}\n{\"id\":2,\"tags\":[\"k\",\"m\"]}\n{\"id\":3,\"tags\":[\"a\",\"b\",\"c\"]}\n";
    assert_eq!(formatted, expected);
}

#[test]
fn test_jsonl_without_sort_preserves_line_order() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("no_sort.jsonl");

    let unformatted = "{\"id\":3}\n{\"id\":1}\n{\"id\":2}\n";
    fs::write(&test_file, unformatted).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg(&test_file)
        .output()
        .unwrap();

    assert!(output.status.success());

    let formatted = fs::read_to_string(&test_file).unwrap();
    let expected = "{\"id\":3}\n{\"id\":1}\n{\"id\":2}\n";
    assert_eq!(formatted, expected);
}

#[test]
fn test_invalid_jsonl_reports_line_number() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("bad.jsonl");

    // Second line is invalid JSON.
    let content = "{\"a\":1}\nnot valid json\n{\"b\":2}\n";
    fs::write(&test_file, content).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg(&test_file)
        .output()
        .unwrap();

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("At line 2"));
}

#[test]
fn test_multiple_files_with_error_continues() {
    let temp_dir = TempDir::new().unwrap();
    let good_file = temp_dir.path().join("good.json");
    let bad_file = temp_dir.path().join("bad.json");

    fs::write(&good_file, r#"{"key":"value"}"#).unwrap();
    fs::write(&bad_file, "not valid json").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_jsonf"))
        .arg(&bad_file)
        .arg(&good_file)
        .output()
        .unwrap();

    // Should fail overall because one file had an error
    assert!(!output.status.success());

    // But the good file should still be formatted
    let formatted = fs::read_to_string(&good_file).unwrap();
    assert_eq!(formatted, "{\n  \"key\": \"value\"\n}\n");
}
