use deunicode::deunicode;
use regex::Regex;

/// Convert a filename to a slugified version based on custom rules
pub fn slugify(name: &str, lowercase: bool) -> String {
    let mut s = deunicode(name);

    // Trim spaces around - and _
    let re_trim = Regex::new(r"\s*([-_])\s*").unwrap();
    s = re_trim.replace_all(&s, "$1").to_string();

    // Replace spaces with dots
    s = s.replace(' ', ".");

    // Remove all unwanted characters (keep word chars, ., -, _)
    // let re_clean = Regex::new(r"[^\w.\-_]+").unwrap();
    let re_clean = Regex::new(r"[^\w.\-{}]+").unwrap();
    s = re_clean.replace_all(&s, "").to_string();

    // Collapse multiple dots
    let re_dots = Regex::new(r"\.{2,}").unwrap();
    s = re_dots.replace_all(&s, ".").to_string();

    // Remove leading and trailing dots
    s = s.trim_matches('.').to_string();

    if lowercase {
        s = s.to_lowercase();
    }

    s
}
