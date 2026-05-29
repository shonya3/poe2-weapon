use std::{fs, path::Path};

use parser::Parsed;
use serde::{Deserialize, Serialize};
#[allow(unused)]
use weapon::{
    AttackSpeedModifier, DamageType, Explicits, FlatDamage, PhysModifier, Quality, Range, Rune,
};
use weapon::{ItemClass, Weapon};

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
    let weapon = Parsed {
        base: "Cinderbark Talisman".to_string(),
        item_class: ItemClass::Talismans,
        quality: Quality(0),
        explicits: Explicits {
            flats: vec![
                FlatDamage {
                    damage_type: DamageType::Fire,
                    range: Range(9, 16),
                },
                FlatDamage {
                    damage_type: DamageType::Cold,
                    range: Range(8, 14),
                },
            ],
            phys: Some(PhysModifier(54)),
            atk_spd: None,
        },
        runes: vec![Rune::LesserGlacial],
    };

    let fixture = Fixture {
        text: "Item Class: Talismans
Rarity: Rare
Kraken Cloak
Cinderbark Talisman
--------
Physical Damage: 18-39 (augmented)
Elemental Damage: 14-26 (fire), 11-19 (cold)
Critical Hit Chance: 8.00%
Attacks per Second: 1.20
--------
Requires: Level 10, 15 Str, 11 Int
--------
Sockets: S 
--------
Item Level: 11
--------
Adds 3 to 5 Cold Damage (rune)
--------
59% increased Flammability Magnitude (implicit)
--------
54% increased Physical Damage
Adds 9 to 16 Fire Damage
Adds 8 to 14 Cold Damage
+7 to Intelligence
12% increased Stun Duration
"
        .to_string(),
        expected: weapon,
    };

    std::fs::write(
        "tests/fixtures/basic_talisman.yaml",
        serde_yaml::to_string(&fixture).unwrap(),
    )
    .unwrap();

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
