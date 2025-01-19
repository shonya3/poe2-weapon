use std::{fs, path::Path};

use parser::Parsed;
use serde::{Deserialize, Serialize};
use weapon::{AttackSpeedModifier, DamageType, FlatDamage, PhysModifier, Quality, Range};
use weapon::{Explicits, Rune};

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
        base: "Expert Shortbow".to_owned(),
        explicits: Explicits {
            flats: vec![FlatDamage {
                damage_type: DamageType::Physical,
                range: Range(12, 18),
            }],
            phys: Some(PhysModifier(118)),
            atk_spd: Some(AttackSpeedModifier(19)),
        },
        runes: vec![Rune::Iron, Rune::Iron],
        quality: Quality(20),
    };

    print_yaml(&Fixture {
        text: "Item Class: Bows
Rarity: Rare
Woe Fletch
Expert Shortbow
--------
Quality: +20% (augmented)
Physical Damage: 164-291 (augmented)
Critical Hit Chance: 5.00%
Attacks per Second: 1.49 (augmented)
--------
Requirements:
Level: 67
Dex: 174
--------
Sockets: S S
--------
Item Level: 75
--------
40% increased Physical Damage (rune)
--------
118% increased Physical Damage
Adds 12 to 18 Physical Damage
+133 to Accuracy Rating
+21% to Critical Damage Bonus
19% increased Attack Speed
Grants 4 Life per Enemy Hit
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
