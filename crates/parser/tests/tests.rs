use std::{fs, path::Path};

use parser::Parsed;
use serde::{Deserialize, Serialize};
#[allow(unused)]
use weapon::{
    AttackSpeedModifier, DamageType, Explicits, FlatDamage, PhysModifier, Quality, Range, Rune,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Fixture {
    pub text: String,
    pub expected: Parsed,
}

impl Fixture {
    pub fn assert(&self) {
        assert_eq!(self.expected, parser::parse(&self.text).unwrap())
    }
}

pub fn print_yaml(fixture: &Fixture) {
    println!("{}", serde_yaml::to_string(&fixture).unwrap());
}

#[test]
fn parser() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let fixtures_dir = Path::new(&manifest_dir).join("tests").join("fixtures");

    fs::read_dir(fixtures_dir)
        .unwrap()
        .map(|file| file.unwrap().path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("yaml"))
        .for_each(|path| {
            let s = fs::read_to_string(path).unwrap();
            let fixture: Fixture = serde_yaml::from_str(&s).unwrap();
            fixture.assert();
        });
}
