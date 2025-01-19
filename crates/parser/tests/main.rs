use std::{fs, path::Path};

use parser::Parsed;
use serde::{Deserialize, Serialize};
use weapon::Explicits;
use weapon::{AttackSpeedModifier, DamageType, FlatDamage, PhysModifier, Quality, Range};

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

pub const S_S: &str = "Item Class: Two Hand Maces
Rarity: Magic
Temple Maul of the Thirsty
--------
Physical Damage: 35-72
Critical Hit Chance: 5.00%
Attacks per Second: 1.20
--------
Requirements:
Level: 28
Str: 65
--------
Item Level: 41
--------
Leeches 4.31% of Physical Damage as Mana";

fn print_yaml(fixture: &Fixture) {
    println!("{}", serde_yaml::to_string(&fixture).unwrap());
}

#[test]
fn yaml() {
    let expected = Parsed {
        base: "Leaden Greathammer".to_owned(),
        explicits: Explicits {
            flats: vec![FlatDamage {
                damage_type: DamageType::Fire,
                range: Range(7, 16),
            }],
            phys: Some(PhysModifier(107)),
            atk_spd: Some(AttackSpeedModifier(9)),
        },
        runes: vec![],
        quality: Quality(20),
    };

    print_yaml(&Fixture {
        text: "Item Class: Two Hand Maces
Rarity: Rare
Plague Crusher
Leaden Greathammer
--------
Quality: +20% (augmented)
Physical Damage: 144-194 (augmented)
Fire Damage: 7-16 (augmented)
Critical Hit Chance: 5.00%
Attacks per Second: 1.20 (augmented)
--------
Requirements:
Level: 33
Str: 76
--------
Item Level: 33
--------
107% increased Physical Damage
Adds 7 to 16 Fire Damage
+88 to Accuracy Rating
+16% to Critical Damage Bonus
9% increased Attack Speed
10% increased Light Radius
"
        .to_owned(),
        expected,
    });
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
