use parser::Parsed;
use serde::{Deserialize, Serialize};
use weapon::{AttackSpeedModifier, DamageType, Explicit, FlatDamage, PhysModifier, Quality, Range};

pub const INPUT_1: &str = "Item Class: Two Hand Maces
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
";

#[derive(Debug, Serialize, Deserialize)]
pub struct Fixture {
    pub clipboard_input: &'static str,
    pub expected: Parsed,
}

impl Fixture {
    pub fn assert(&self) {
        assert_eq!(self.expected, parser::parse(self.clipboard_input))
    }
}

#[test]
fn parser() {
    let expected: Parsed = Parsed {
        base: "Leaden Greathammer".to_owned(),
        explicits: vec![
            Explicit::Phys(PhysModifier(107)),
            Explicit::Flat(FlatDamage {
                damage_type: DamageType::Fire,
                value: Range(7, 16),
            }),
            Explicit::AtkSpd(AttackSpeedModifier(9)),
        ],
        runes: vec![],
        quality: Quality(20),
    };

    let fixture = Fixture {
        clipboard_input: INPUT_1,
        expected,
    };

    println!("{}", serde_yaml::to_string(&fixture).unwrap());

    // let s = serde_json::to_string(&expected).unwrap();
    // println!("{s}");

    // let s = serde_yaml::to_string(&expected).unwrap();
    // println!("{s}");

    // assert_eq!()
}
