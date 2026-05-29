use std::{fs, path::Path};

#[test]
fn parser() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let fixtures_dir = Path::new(&manifest_dir).join("tests").join("fixtures");

    let mut dirs: Vec<_> = fs::read_dir(fixtures_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();

    for dir in dirs {
        let text = fs::read_to_string(dir.join("input.txt")).unwrap();
        let expected: parser::Parsed =
            serde_json::from_str(&fs::read_to_string(dir.join("expected.json")).unwrap()).unwrap();
        assert_eq!(expected, parser::parse(&text).unwrap());
    }
}
