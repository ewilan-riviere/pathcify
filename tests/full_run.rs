use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) {
    let src = src.as_ref();
    let dst = dst.as_ref();

    fs::create_dir_all(dst).unwrap();

    for entry in walkdir::WalkDir::new(src) {
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

fn run_pathcify(target: &Path, lowercase: bool) -> std::process::ExitStatus {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pathcify"));
    cmd.arg(target);

    if lowercase {
        cmd.arg("--lowercase");
    }

    cmd.status().expect("failed to run pathcify")
}

fn print_tree(path: &Path, prefix: &str) {
    if path.is_dir() {
        println!(
            "{}{}/",
            prefix,
            path.file_name().unwrap_or_default().to_string_lossy()
        );
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            print_tree(&entry.path(), &format!("{}  ", prefix));
        }
    } else {
        println!(
            "{}{}",
            prefix,
            path.file_name().unwrap_or_default().to_string_lossy()
        );
    }
}

#[test]
fn test_full_run_original_case() {
    let source_path = PathBuf::from("tests/data");
    let output_path = PathBuf::from("tests/output");

    if output_path.exists() {
        fs::remove_dir_all(&output_path).unwrap();
    }

    copy_dir_all(&source_path, &output_path);
    let status = run_pathcify(&output_path, false);
    assert!(status.success(), "pathcify failed without --lowercase");

    println!("\n=== Final directory tree (original case) ===");
    print_tree(&output_path, "");

    // Update these assertions based on expected final paths
    assert!(output_path
        .join("Spaced.Repository")
        .join("Noooooooh")
        .join("Im.Your.Father")
        .exists());
    assert!(output_path.join("ThisIsalooping.md").exists());
}

#[test]
fn test_full_run_lowercase() {
    let source_path = PathBuf::from("tests/data");
    let output_path = PathBuf::from("tests/output_lowercase");

    if output_path.exists() {
        fs::remove_dir_all(&output_path).unwrap();
    }

    copy_dir_all(&source_path, &output_path);
    let status = run_pathcify(&output_path, true);
    assert!(status.success(), "pathcify failed with --lowercase");

    println!("\n=== Final directory tree (lowercase) ===");
    print_tree(&output_path, "");

    // Update these assertions based on expected final paths
    assert!(output_path
        .join("spaced.repository")
        .join("noooooooh")
        .join("im.your.father")
        .exists());
    assert!(output_path.join("thisisalooping.md").exists());
}
