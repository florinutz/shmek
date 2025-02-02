mod integration_common;
use std::fs;
use tempfile::TempDir;
use yek::serialize_repo;
use yek::YekConfig;

#[test]
fn skips_known_binary_files() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("yek-output");
    fs::create_dir_all(&output_dir).unwrap();

    // Create a binary file
    let test_file = temp.path().join("test.jpg");
    fs::write(&test_file, b"binary content").unwrap();

    // Create a text file
    let text_file = temp.path().join("test.txt");
    fs::write(&text_file, "text content").unwrap();

    // Run serialization
    let mut config = YekConfig::default();
    config.output_dir = Some(output_dir.clone());
    serialize_repo(temp.path(), Some(&config)).unwrap();

    // Check that the first chunk exists and contains only the text file
    let chunk_0 = output_dir.join("chunk-0.txt");
    assert!(chunk_0.exists(), "Should write first chunk");
    let content = fs::read_to_string(chunk_0).unwrap();
    assert!(
        content.contains("text content"),
        "Should contain text file content"
    );
    assert!(
        !content.contains("binary content"),
        "Should not contain binary file content"
    );
}

#[test]
fn respects_custom_binary_extensions() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("yek-output");
    fs::create_dir_all(&output_dir).unwrap();

    // Create a file with custom binary extension
    let test_file = temp.path().join("test.dat");
    fs::write(&test_file, "custom binary content").unwrap();

    // Create a text file
    let text_file = temp.path().join("test.txt");
    fs::write(&text_file, "text content").unwrap();

    // Run serialization with custom config
    let mut config = YekConfig::default();
    config.output_dir = Some(output_dir.clone());
    config.binary_extensions = vec!["dat".to_string()];
    serialize_repo(temp.path(), Some(&config)).unwrap();

    // Check that the first chunk exists and contains only the text file
    let chunk_0 = output_dir.join("chunk-0.txt");
    assert!(chunk_0.exists(), "Should write first chunk");
    let content = fs::read_to_string(chunk_0).unwrap();
    assert!(
        content.contains("text content"),
        "Should contain text file content"
    );
    assert!(
        !content.contains("custom binary content"),
        "Should not contain binary file content"
    );
}
