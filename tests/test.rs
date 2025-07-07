use std::fs::{self, File};
use tempfile::tempdir;

use pathcify::slug::slugify;

#[test]
fn test_slugify_examples() {
    let input = "La Quête d'Ewilan vol.1 : D'un monde à l'autre-·/_,:; (1), [Bottero, Pierre]Author @{1} <book> ?!//&";
    // let expected = "la.quete.dewilan.vol.1.dun.monde.a.lautre-._.1.bottero.pierre.author.{1}.book";
    let expected = "la.quete.dewilan.vol.1.dun.monde.a.lautre-_.1.bottero.pierreauthor.{1}.book";
    let output = slugify(input, true);
    assert_eq!(output, expected);

    assert_eq!(slugify("00 - Préface", false), "00-Preface");
    assert_eq!(slugify("Góðan Daginn", false), "Godan.Daginn");
}

#[test]
fn test_recursive_renaming() {
    let temp = tempdir().unwrap();
    let root = temp.path().join("Test Dir");
    fs::create_dir_all(&root).unwrap();

    let subdir = root.join("Góðan daginn");
    fs::create_dir(&subdir).unwrap();

    let file_path = subdir.join("00 - Préface.txt");
    File::create(&file_path).unwrap();

    // Run renaming
    pathcify::walker::process_dir(&temp.path(), true, true);

    let renamed_root = temp.path().join("test.dir");
    let renamed_subdir = renamed_root.join("godan.daginn");
    let renamed_file = renamed_subdir.join("00-preface.txt");

    assert!(renamed_root.exists());
    assert!(renamed_subdir.exists());
    assert!(renamed_file.exists());
}
