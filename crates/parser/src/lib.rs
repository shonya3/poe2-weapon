#![allow(unused)]
pub mod bases;

use std::str::FromStr;

use bases::BASES;
use serde::{Deserialize, Serialize};
use weapon::{
    AttackSpeedModifier, DamageType, Explicits, FlatDamage, ItemClass, PhysModifier, Quality,
    Range, Rune, RuneMartialBonus, Weapon, WEAPON_STATS,
};

pub const SUPPORTED_ITEM_CLASSES: [&str; 6] = [
    "One Hand Maces",
    "Two Hand Maces",
    "Quarterstaves",
    "Bows",
    "Crossbows",
    "Spears",
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
    let item_class = text
        .lines()
        .find(|s| s.contains("Item Class:"))
        .and_then(|s| {
            let (_, right) = s.split_once(": ")?;
            Some(right.trim())
        })
        .or_else(|| {
            WEAPON_STATS
                .iter()
                .find(|s| s.base == base)
                .map(|s| s.item_class.as_str())
        })
        .ok_or(ParseError::ItemClassMissing)?;

    if !SUPPORTED_ITEM_CLASSES.contains(&item_class) {
        return Err(ParseError::UnsupportedItemClass(item_class.to_owned()));
    }

    let item_class = serde_json::from_str::<ItemClass>(&format!("\"{item_class}\"")).unwrap();

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
    let original_line = line.trim();
    if !original_line.ends_with("(rune)") {
        return None;
    }
    let mod_line = original_line.replace("(rune)", "").trim().to_string();

    // Attempt to parse as Physical Damage Modifier
    if let Some(parsed_phys_mod) = try_parse_phys_modifier(&mod_line) {
        let phys_val = parsed_phys_mod.0;

        let greater_iron_val = Rune::greater_iron_rune_martial().0;
        let iron_val = Rune::iron_rune_martial().0;
        let lesser_iron_val = Rune::lesser_iron_rune_martial().0;

        if greater_iron_val > 0 && phys_val % greater_iron_val == 0 {
            let count = (phys_val / greater_iron_val) as usize;
            if count > 0 {
                return Some(vec![Rune::GreaterIron; count]);
            }
        }
        if iron_val > 0 && phys_val % iron_val == 0 {
            let count = (phys_val / iron_val) as usize;
            if count > 0 {
                return Some(vec![Rune::Iron; count]);
            }
        }
        if lesser_iron_val > 0 && phys_val % lesser_iron_val == 0 {
            let count = (phys_val / lesser_iron_val) as usize;
            if count > 0 {
                return Some(vec![Rune::LesserIron; count]);
            }
        }

        return None;

    // Attempt to parse as Flat Damage Modifier
    } else if let Some(parsed_flat_mod) = try_parse_flat_damage(&mod_line) {
        // Define the base rune tiers and their corresponding enum variants.
        // Order might matter if there's ambiguity, typically checking larger base runes first for multiples.
        let candidate_rune_tiers = [
            (Rune::greater_desert_rune_martial(), Rune::GreaterDesert),
            (Rune::desert_rune_martial(), Rune::Desert),
            (Rune::lesser_desert_rune_martial(), Rune::LesserDesert),
            (Rune::greater_glacial_rune_martial(), Rune::GreaterGlacial),
            (Rune::glacial_rune_martial(), Rune::Glacial),
            (Rune::lesser_glacial_rune_martial(), Rune::LesserGlacial),
            (Rune::greater_storm_rune_martial(), Rune::GreaterStorm),
            (Rune::storm_rune_martial(), Rune::Storm),
            (Rune::lesser_storm_rune_martial(), Rune::LesserStorm),
        ];

        // Stage 1: Try for an exact single rune match
        for (tier_stats, rune_variant) in &candidate_rune_tiers {
            if tier_stats.damage_type == parsed_flat_mod.damage_type
                && tier_stats.range == parsed_flat_mod.range
            {
                return Some(vec![*rune_variant]); // Exact match for one rune
            }
        }

        // Stage 2: If no exact single rune match, try to parse as multiples of a base rune tier
        // We iterate in the same order (Greater, Normal, Lesser for each element type)
        // to give precedence to multiples of larger runes if ambiguity were possible.
        for (tier_stats, rune_variant) in &candidate_rune_tiers {
            if tier_stats.damage_type == parsed_flat_mod.damage_type {
                let tier_min = tier_stats.range.0;
                let tier_max = tier_stats.range.1;
                let parsed_min = parsed_flat_mod.range.0;
                let parsed_max = parsed_flat_mod.range.1;

                // Ensure the base tier rune has valid damage values to avoid division by zero or logical errors.
                if tier_min > 0 && tier_max > 0 {
                    // Min damage typically > 0 for elemental runes
                    if parsed_min % tier_min == 0 && parsed_max % tier_max == 0 {
                        let count_from_min = parsed_min / tier_min;
                        let count_from_max = parsed_max / tier_max;

                        // Both min and max damage must scale by the same factor, and that factor must be greater than 0.
                        if count_from_min == count_from_max && count_from_min > 0 {
                            // If count is 1, it should have been caught by the exact match (Stage 1).
                            // This block is for actual multiples (count > 1).
                            if count_from_min > 0 {
                                // count_from_min == 1 should be caught by Stage 1
                                return Some(vec![*rune_variant; count_from_min as usize]);
                            }
                        }
                    }
                }
            }
        }

        return None; // No matching elemental rune tier or multiple found
    }

    None // Line is not a recognized rune modifier
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
            super::try_parse_rune("32% increased Physical Damage (rune)")
        )
    }
}
