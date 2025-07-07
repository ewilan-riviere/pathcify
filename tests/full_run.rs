use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

/// Recursively copies the contents of `src` to `dst`
fn copy_dir_all(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in WalkDir::new(src) {
        let entry = entry.unwrap();
        let rel_path = entry.path().strip_prefix(src).unwrap();
        let dest_path = dst.join(rel_path);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&dest_path).unwrap();
        } else {
            fs::copy(entry.path(), &dest_path).unwrap();
        }
    }
}

/// Deletes a directory if it exists
fn clean_dir(path: &Path) {
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
}

#[test]
fn test_full_run_original_case() {
    let data_path = PathBuf::from("tests/data");
    let output_path = PathBuf::from("tests/output");

    clean_dir(&output_path);
    copy_dir_all(&data_path, &output_path);

    let status = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .arg(&output_path)
        .status()
        .expect("Failed to run dotify without lowercase");

    assert!(status.success(), "dotify failed without --lowercase");

    assert!(output_path
        .join("@")
        .join("HelloIceland")
        .join("LoLyOuHaVeAKoIFiSh")
        .exists());
    assert!(output_path
        .join("Spaced.Repository")
        .join("Alien")
        .join("Ripley.txt")
        .exists());
    assert!(output_path
        .join("Spaced.Repository")
        .join("Noooooooh")
        .join("Im.Your.Father")
        .exists());
    assert!(output_path
        .join("Spaced.Repository")
        .join("Goutez.A.La.Gamme.Energie")
        .exists());
    assert!(output_path.join("ThisIsalooping.md").exists());
}

#[test]
fn test_full_run_lowercase() {
    let data_path = PathBuf::from("tests/data");
    let output_path = PathBuf::from("tests/output_lowercase");

    clean_dir(&output_path);
    copy_dir_all(&data_path, &output_path);

    let status = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .arg("--lowercase")
        .arg(&output_path)
        .status()
        .expect("Failed to run dotify with lowercase");

    assert!(status.success(), "dotify failed with --lowercase");

    assert!(output_path
        .join("@")
        .join("helloiceland")
        .join("lolyouhaveakoifish")
        .exists());
    assert!(output_path
        .join("spaced.repository")
        .join("alien")
        .join("ripley.txt")
        .exists());
    assert!(output_path
        .join("spaced.repository")
        .join("noooooooh")
        .join("im.your.father")
        .exists());
    assert!(output_path
        .join("spaced.repository")
        .join("alien")
        .join("ripley.txt")
        .exists());
    assert!(output_path
        .join("spaced.repository")
        .join("goutez.a.la.gamme.energie")
        .exists());
    assert!(output_path.join("thisisalooping.md").exists());
}

#[test]
fn test_slugify_conflict() {
    let data_path = PathBuf::from("tests/data");
    let output_path = PathBuf::from("tests/output_conflict");

    // Clean and copy data
    clean_dir(&output_path);
    copy_dir_all(&data_path, &output_path);

    // Create conflicting file
    let conflict_dir = output_path.join("spaced.repository");
    fs::create_dir_all(&conflict_dir).unwrap();

    // File that already exists with slugified name
    let existing_file_path = conflict_dir.join("conflict.file.txt");
    let mut file = fs::File::create(&existing_file_path).unwrap();
    writeln!(file, "Existing file content").unwrap();

    // File that will slugify to same name
    let conflict_file_path = conflict_dir.join("Conflict File.txt");
    let mut conflict_file = fs::File::create(&conflict_file_path).unwrap();
    writeln!(conflict_file, "New conflicting file content").unwrap();

    // Run dotify
    let status = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .arg("--lowercase")
        .arg(&output_path)
        .status()
        .expect("Failed to run dotify");

    assert!(status.success());

    // Assert the existing file is still there
    assert!(existing_file_path.exists());

    // The conflict file should NOT be renamed because slug exists, so original stays
    assert!(conflict_file_path.exists());

    // Optionally check the conflicting file was not renamed
    let slugified_conflict_path = conflict_dir.join("conflict.file.txt");
    assert!(slugified_conflict_path.exists());
}
