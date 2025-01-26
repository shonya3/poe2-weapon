#![allow(unused)]
pub mod bases;

use std::str::FromStr;

use bases::BASES;
use serde::{Deserialize, Serialize};
use weapon::{
    AttackSpeedModifier, DamageType, Explicits, FlatDamage, ItemClass, PhysModifier, Quality,
    Range, Rune, Weapon,
};

pub const SUPPORTED_ITEM_CLASSES: [&str; 5] = [
    "One Hand Maces",
    "Two Hand Maces",
    "Quarterstaves",
    "Bows",
    "Crossbows",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parsed {
    pub base: String,
    pub item_class: ItemClass,
    pub explicits: Explicits,
    pub runes: Vec<Rune>,
    pub quality: Quality,
}

impl Parsed {
    pub fn into_weapon(self) -> Weapon {
        self.into()
    }
}

impl From<Parsed> for Weapon {
    fn from(value: Parsed) -> Self {
        Weapon {
            base: value.base,
            item_class: value.item_class,
            quality: value.quality,
            explicits: value.explicits,
            runes: value.runes,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ItemClassMissing,
    UnsupportedItemClass(String),
    UnsupportedItemBase,
}

pub fn parse(text: &str) -> Result<Parsed, ParseError> {
    let text = text
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>()
        .join("\n");

    let item_class = text
        .lines()
        .find(|s| s.contains("Item Class:"))
        .and_then(|s| {
            let (_, right) = s.split_once(": ")?;
            Some(right.trim())
        })
        .ok_or(ParseError::ItemClassMissing)?;
    if !SUPPORTED_ITEM_CLASSES.contains(&item_class) {
        return Err(ParseError::UnsupportedItemClass(item_class.to_owned()));
    }

    let item_class = serde_json::from_str::<ItemClass>(&format!("\"{item_class}\"")).unwrap();

    let mut base: Option<String> = None;
    let mut quality = Quality::default();
    let mut phys: Option<PhysModifier> = None;
    let mut atk_spd: Option<AttackSpeedModifier> = None;
    let mut flats: Vec<FlatDamage> = vec![];

    let mut item_level_line_met = false;
    let mut lines = text.lines();

    for line in &mut lines {
        if base.is_none() {
            if let Some(b) = BASES.iter().rev().find(|b| line.contains(*b)) {
                base = Some(b.to_string());
                continue;
            }
        }

        if quality.0 != 0 {
            break;
        }
        if let Some(q) = try_parse_quality(line) {
            quality = q;
        }

        if line.starts_with("Item Level") {
            item_level_line_met = true;
            break;
        }
    }

    let base = base.ok_or(ParseError::UnsupportedItemBase)?;
    let mut runes: Vec<Rune> = vec![];

    if !item_level_line_met {
        for line in lines.skip_while(|s| !s.starts_with("Item Level")) {
            if atk_spd.is_some() {
                break;
            }

            if phys.is_none() {
                // Runes come right before explicits. Phys is our first explicit.
                if let Some(line_runes) = try_parse_rune(line) {
                    runes.extend(line_runes);
                    continue;
                }

                if let Some(p) = try_parse_phys_modifier(line) {
                    phys = Some(p);
                    continue;
                }
            }

            if let Some(flat) = try_parse_flat_damage(line) {
                flats.push(flat);
                continue;
            }

            if let Some(aspd) = try_parse_attack_speed_modifier(line) {
                atk_spd = Some(aspd)
            }
        }
    } else {
        for line in lines {
            if atk_spd.is_some() {
                break;
            }
            if phys.is_none() {
                // Runes come right before explicits. Phys is our first explicit.
                if let Some(line_runes) = try_parse_rune(line) {
                    runes.extend(line_runes);
                    continue;
                }

                if let Some(p) = try_parse_phys_modifier(line) {
                    phys = Some(p);
                    continue;
                }
            }

            if let Some(flat) = try_parse_flat_damage(line) {
                flats.push(flat);
                continue;
            }

            if let Some(aspd) = try_parse_attack_speed_modifier(line) {
                atk_spd = Some(aspd)
            }
        }
    }

    Ok(Parsed {
        base,
        item_class,
        explicits: Explicits {
            flats,
            phys,
            atk_spd,
        },
        runes,
        quality,
    })
}

fn try_parse_rune(line: &str) -> Option<Vec<Rune>> {
    let line = line.trim();
    if !line.ends_with("(rune)") {
        return None;
    }
    let line = line.replace("(rune)", "");
    let line = line.trim();

    if let Some(phys) = try_parse_phys_modifier(line) {
        let length = (phys.0 as f32 / Rune::iron_rune_martial().0 as f32).ceil() as usize;
        return Some(vec![Rune::Iron; length]);
    }

    if let Some(flat) = try_parse_flat_damage(line) {
        let (rune, max) = match flat.damage_type {
            DamageType::Physical => panic!("Never happens: No rune with flat phys"),
            DamageType::Fire => (Rune::Desert, Rune::desert_rune_martial().range.1),
            DamageType::Cold => (Rune::Glacial, Rune::glacial_rune_martial().range.1),
            DamageType::Lightning => (Rune::Storm, Rune::storm_rune_martial().range.1),
            DamageType::Chaos => panic!("Never happens: No rune with flat chaos"),
        };

        let length = (flat.range.1 as f32 / max as f32).ceil() as usize;
        return Some(vec![rune; length]);
    }

    None
}

/// Try find Adds 7 to 16 Fire Damage
fn try_parse_flat_damage(line: &str) -> Option<FlatDamage> {
    let mut iter = line.split(" ");
    if iter.next() != Some("Adds") {
        return None;
    }

    let Ok(min) = iter.next().unwrap_or_default().parse::<u16>() else {
        return None;
    };

    if iter.next() != Some("to") {
        return None;
    }

    let Ok(max) = iter.next().unwrap_or_default().parse::<u16>() else {
        return None;
    };

    let damage_type = match iter.next().unwrap_or_default() {
        "Fire" => DamageType::Fire,
        "Cold" => DamageType::Cold,
        "Lightning" => DamageType::Lightning,
        "Chaos" => DamageType::Chaos,
        "Physical" => DamageType::Physical,
        _ => return None,
    };

    if iter.next() != Some("Damage") {
        return None;
    }

    if iter.next().is_some() {
        return None;
    }

    Some(FlatDamage {
        damage_type,
        range: Range(min, max),
    })
}

/// 107% increased Physical Damage
fn try_parse_phys_modifier(line: &str) -> Option<PhysModifier> {
    let mut iter = line.split(" ");
    let value = iter.next().and_then(|s| {
        if !s.ends_with("%") {
            return None;
        }

        s.replace("%", "").parse::<u16>().ok()
    })?;

    if iter.next() == Some("increased")
        && iter.next() == Some("Physical")
        && iter.next() == Some("Damage")
        && iter.next().is_none()
    {
        return Some(PhysModifier(value));
    }

    None
}

fn try_parse_attack_speed_modifier(line: &str) -> Option<AttackSpeedModifier> {
    let mut iter = line.split(" ");
    let value = iter.next().and_then(|s| {
        if !s.ends_with("%") {
            return None;
        }

        s.replace("%", "").parse::<u8>().ok()
    })?;

    if iter.next() == Some("increased")
        && iter.next() == Some("Attack")
        && iter.next() == Some("Speed")
        && iter.next().is_none()
    {
        return Some(AttackSpeedModifier(value));
    }

    None
}

fn try_parse_quality(line: &str) -> Option<Quality> {
    let mut iter = line.split(" ");
    if iter.next() != Some("Quality:") {
        return None;
    }
    let value = iter.next().and_then(|s| {
        if !s.starts_with("+") || !s.ends_with("%") {
            return None;
        }

        s.replace("+", "").replace("%", "").parse::<u8>().ok()
    })?;

    Some(Quality(value))
}

#[cfg(test)]
mod tests {
    use weapon::{AttackSpeedModifier, DamageType, FlatDamage, PhysModifier, Quality, Range, Rune};

    #[test]
    fn try_parse_flat_damage() {
        assert_eq!(
            Some(FlatDamage {
                damage_type: DamageType::Fire,
                range: Range(7, 16),
            }),
            super::try_parse_flat_damage("Adds 7 to 16 Fire Damage")
        )
    }

    #[test]
    fn try_parse_phys_modifier() {
        assert_eq!(
            Some(PhysModifier(107)),
            super::try_parse_phys_modifier("107% increased Physical Damage")
        );
    }

    #[test]
    fn try_parse_quality() {
        assert_eq!(
            Some(Quality(20)),
            super::try_parse_quality("Quality: +20% (augmented)")
        )
    }

    #[test]
    fn try_parse_attack_speed_modifier() {
        assert_eq!(
            Some(AttackSpeedModifier(9)),
            super::try_parse_attack_speed_modifier("9% increased Attack Speed")
        );
    }

    #[test]
    fn try_parse_rune() {
        assert_eq!(
            Some(vec![Rune::Iron, Rune::Iron]),
            super::try_parse_rune("40% increased Physical Damage (rune)")
        )
    }
}
