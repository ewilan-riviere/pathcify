use std::fs::{self, File};
use std::path::PathBuf;
use std::process::Command;

/// Helper to copy test data into a clean test dir
fn prepare_test_dir(name: &str) -> PathBuf {
    let base = PathBuf::from(format!("tests/{}", name));
    if base.exists() {
        fs::remove_dir_all(&base).unwrap();
    }
    fs::create_dir_all(&base).unwrap();
    base
}

/// Run the compiled binary on the target directory
fn run_pathcify(target: &PathBuf) {
    let status = Command::new(env!("CARGO_BIN_EXE_pathcify"))
        .arg(target)
        .status()
        .expect("Failed to run pathcify");

    assert!(status.success());
}

#[test]
fn test_skips_ds_store_file() {
    // Setup
    let path = prepare_test_dir("ds_store_test");
    let hidden_file = path.join(".DS_Store");
    let visible_file = path.join("Visible File.txt");

    // Create test files
    File::create(&hidden_file).unwrap();
    File::create(&visible_file).unwrap();

    // Run
    run_pathcify(&path);

    // Check: .DS_Store should still exist (untouched)
    assert!(hidden_file.exists(), ".DS_Store should be skipped");

    // Check: visible file should be renamed
    let expected = path.join("Visible.File.txt");
    assert!(expected.exists(), "Visible File.txt should be renamed");
}
