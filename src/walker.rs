use crate::slug::slugify;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

const SKIP_FILENAMES: &[&str] = &[
    ".DS_Store",
    "desktop.ini",
    "Thumbs.db",
    "__MACOSX",
    ".AppleDouble",
];

fn safe_rename(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.exists() && dst.exists() {
        // Compare file names case-insensitively
        let src_name = src.file_name().and_then(OsStr::to_str).unwrap_or("");
        let dst_name = dst.file_name().and_then(OsStr::to_str).unwrap_or("");

        if src_name.eq_ignore_ascii_case(dst_name) {
            // Case-only rename on case-insensitive FS: do two-step rename
            let tmp = src.with_extension("tmp_rename");
            fs::rename(src, &tmp)?;
            fs::rename(&tmp, dst)?;
            return Ok(());
        }
    }

    fs::rename(src, dst)
}

/// Recursively rename files and directories in place using slugify rules
pub fn process_dir(path: &Path, lowercase: bool) {
    for entry in WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        let original_path = entry.path();

        if let Some(name) = original_path.file_name().and_then(|n| n.to_str()) {
            let slug = slugify(name, lowercase);

            if slug == name {
                continue;
            }

            if SKIP_FILENAMES.contains(&name) {
                continue;
            }

            let parent = original_path.parent().unwrap();
            let new_path = parent.join(&slug);

            if new_path.exists() {
                eprintln!("Skipping: {} → {} (already exists)", name, slug);
                continue;
            }

            if let Err(e) = safe_rename(original_path, &new_path) {
                eprintln!("Failed to rename {}: {}", name, e);
            } else {
                println!("Renamed: {} → {}", name, slug);
            }
        }
    }
}
