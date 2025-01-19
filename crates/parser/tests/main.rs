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

pub const TEXT: &str = "Item Class: Bows
Rarity: Unique
Splinterheart
Recurve Bow
--------
Quality: +20% (augmented)
Physical Damage: 34-71 (augmented)
Fire Damage: 7-11 (augmented)
Critical Hit Chance: 5.00%
Attacks per Second: 1.10
--------
Requirements:
Level: 16
Dex: 38
--------
Sockets: S S 
--------
Item Level: 53
--------
20% increased Physical Damage (rune)
Adds 7 to 11 Fire Damage (rune)
--------
71% increased Physical Damage
+58 to Accuracy Rating
24% increased Projectile Speed
Projectiles Split towards +2 targets
--------
The forests of the Vastiri held many secrets
mystical and dark. Men learned not to wander,
lest they return with a strange new purpose.
";

fn print_yaml(fixture: &Fixture) {
    println!("{}", serde_yaml::to_string(&fixture).unwrap());
}

#[test]
fn yaml() {
    let expected = Parsed {
        base: "Recurve Bow".to_owned(),
        explicits: Explicits {
            flats: vec![],
            phys: Some(PhysModifier(71)),
            atk_spd: None,
        },
        runes: vec![Rune::Desert, Rune::Iron],
        quality: Quality(20),
    };

    print_yaml(&Fixture {
        text: r#"Item Class: Bows
Rarity: Unique
Splinterheart
Recurve Bow
--------
Quality: +20% (augmented)
Physical Damage: 34-71 (augmented)
Fire Damage: 7-11 (augmented)
Critical Hit Chance: 5.00%
Attacks per Second: 1.10
--------
Requirements:
Level: 16
Dex: 38
--------
Sockets: S S 
--------
Item Level: 53
--------
20% increased Physical Damage (rune)
Adds 7 to 11 Fire Damage (rune)
--------
71% increased Physical Damage
+58 to Accuracy Rating
24% increased Projectile Speed
Projectiles Split towards +2 targets
--------
The forests of the Vastiri held many secrets
mystical and dark. Men learned not to wander,
lest they return with a strange new purpose.
"#
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
