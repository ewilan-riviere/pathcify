use std::fs::{self, File};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn print_tree(path: &Path, prefix: &str) {
    if let Ok(entries) = fs::read_dir(path) {
        let entries = entries.filter_map(Result::ok).collect::<Vec<_>>();
        for (i, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let is_last = i == entries.len() - 1;
            let name = path.file_name().unwrap().to_string_lossy();

            // Print the prefix + branch or corner
            println!(
                "{}{}{}",
                prefix,
                if is_last { "└── " } else { "├── " },
                name
            );

            // If directory, recurse with updated prefix
            if path.is_dir() {
                let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                print_tree(&path, &new_prefix);
            }
        }
    }
}

/// Prepare clean test directory, deleting it first if exists
fn prepare_test_dir(name: &str) -> PathBuf {
    let base = PathBuf::from(format!("tests/{}", name));
    if base.exists() {
        fs::remove_dir_all(&base).unwrap();
    }
    fs::create_dir_all(&base).unwrap();
    // Create a .DS_Store file in `tests/data/Spaced Repository` to simulate macOS behavior
    let ds_store_path = base.join("Spaced.Repository").join(".DS_Store");
    fs::create_dir_all(ds_store_path.parent().unwrap()).unwrap();
    File::create(ds_store_path).unwrap();
    base
}

/// Recursively copy directory contents from src to dst
fn copy_dir_all(src: &PathBuf, dst: &PathBuf) {
    fs::create_dir_all(dst).unwrap();
    for entry in walkdir::WalkDir::new(src) {
        let entry = entry.unwrap();
        let path = entry.path();
        let rel_path = path.strip_prefix(src).unwrap();
        let target = dst.join(rel_path);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target).unwrap();
        } else {
            fs::copy(path, &target).unwrap();
        }
    }
}

/// Run the binary with optional `--lowercase` flag
fn run_pathcify(target: &PathBuf, lowercase: bool) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pathcify"));
    cmd.arg(target);
    if lowercase {
        cmd.arg("--lowercase");
    }
    let status = cmd.status().expect("Failed to run pathcify");
    assert!(status.success());
}

#[test]
fn test_full_run_original_case() {
    let data_dir = PathBuf::from("tests/data");
    let output_dir = prepare_test_dir("output");
    copy_dir_all(&data_dir, &output_dir);

    run_pathcify(&output_dir, false);

    // Check expected paths, e.g.
    assert!(output_dir.join("Spaced.Repository").exists());
    assert!(output_dir.join("@").join("HelloIceland").exists());
    assert!(output_dir.join("ThisIsalooping.md").exists());

    println!("Directory structure after pathcify:");
    print_tree(output_dir.as_path(), "");

    // Check .DS_Store is untouched
    assert!(output_dir
        .join("Spaced.Repository")
        .join(".DS_Store")
        .exists());
}

#[test]
fn test_full_run_lowercase() {
    let data_dir = PathBuf::from("tests/data");
    let output_dir = prepare_test_dir("output_lowercase");
    copy_dir_all(&data_dir, &output_dir);

    run_pathcify(&output_dir, true);

    assert!(output_dir.join("spaced.repository").exists());
    assert!(output_dir
        .join("spaced.repository")
        .join("noooooooh")
        .exists());
    assert!(output_dir
        .join("spaced.repository")
        .join("noooooooh")
        .join("im.your.father")
        .exists());
    assert!(output_dir.join("@").join("helloiceland").exists());
    assert!(output_dir.join("thisisalooping.md").exists());

    // .DS_Store should remain
    assert!(output_dir
        .join("spaced.repository")
        .join(".DS_Store")
        .exists());
}

#[test]
fn test_slugify_conflict() {
    let path = prepare_test_dir("conflict_test");
    // Create files that conflict when slugified
    File::create(path.join("Conflict File.txt")).unwrap();
    File::create(path.join("conflict.file.txt")).unwrap();

    run_pathcify(&path, false);

    // Both files still exist, conflict skip expected
    assert!(path.join("Conflict.File.txt").exists());
    assert!(path.join("conflict.file.txt").exists());
}

#[test]
fn test_skips_ds_store_file() {
    let path = prepare_test_dir("ds_store_test");
    let hidden_file = path.join(".DS_Store");
    let visible_file = path.join("Visible File.txt");

    File::create(&hidden_file).unwrap();
    File::create(&visible_file).unwrap();

    run_pathcify(&path, false);

    assert!(hidden_file.exists());
    assert!(path.join("Visible.File.txt").exists());
}
