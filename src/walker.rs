use crate::slug::slugify;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

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

            let parent = original_path.parent().unwrap();
            let new_path = parent.join(&slug);

            if new_path.exists() {
                eprintln!("Skipping: {} → {} (already exists)", name, slug);
                continue;
            }

            if let Err(e) = fs::rename(original_path, &new_path) {
                eprintln!("Failed to rename {}: {}", name, e);
            } else {
                println!("Renamed: {} → {}", name, slug);
            }
        }
    }
}
