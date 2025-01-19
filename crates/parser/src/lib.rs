#![allow(unused)]
pub mod bases;

use bases::BASES;
use serde::{Deserialize, Serialize};
use weapon::{
    AttackSpeedModifier, DamageType, Explicit, FlatDamage, PhysModifier, Quality, Range, Rune,
};

pub const SUPPORTED_ITEM_CLASSES: [&str; 5] = [
    "One Hand Maces",
    "Two Hand Maces",
    "Quarterstaves",
    "Bows",
    "Crossbows",
];

#[derive(Debug)]
pub enum ParseError {
    ItemClassMissing,
    UnsupportedItemClass(String),
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

pub fn parse(text: &str) -> Result<Parsed, ParseError> {
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

    let mut quality = Quality::default();
    let mut attack_speed = AttackSpeedModifier::default();

    for s in text.lines() {
        //
    }

    todo!()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parsed {
    pub base: String,
    pub explicits: Vec<Explicit>,
    pub runes: Vec<Rune>,
    pub quality: Quality,
}

#[cfg(test)]
mod tests {
    use weapon::{DamageType, FlatDamage, Range};

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
}
