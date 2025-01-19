use std::ops::Add;

use serde::{Deserialize, Serialize};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Range(pub u16, pub u16);

impl Add for Range {
    type Output = u16;

    fn add(self, rhs: Self) -> Self::Output {
        self.0 + self.1 + rhs.0 + rhs.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Rune {
    Iron,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub base_damage: Vec<FlatDamage>,
    pub quality: Quality,
    pub attack_speed: f32,
    pub explicits: Vec<Explicit>,
    pub runes: Vec<Rune>,
}

impl Weapon {
    pub fn phys_dps(&self) -> f32 {
        let mut flat_phys_explicit = FlatDamage::default();
        let mut phys_modifier_explicit = PhysModifier::default();
        let mut attack_speed_modifier = AttackSpeedModifier::default();

        self.explicits.iter().for_each(|explicit| match explicit {
            Explicit::Flat(flat_damage) => {
                if matches!(flat_damage.damage_type, DamageType::Physical) {
                    flat_phys_explicit = *flat_damage
                }
            }
            Explicit::Phys(phys) => phys_modifier_explicit = *phys,
            Explicit::AtkSpd(attack_speed) => attack_speed_modifier = *attack_speed,
            Explicit::Other(_) => {}
        });

        let base_phys = self
            .base_damage
            .iter()
            .find(|damage| matches!(damage.damage_type, DamageType::Physical))
            .copied()
            .unwrap_or_default();

        let runes_phys_modifier = PhysModifier(
            20 * self
                .runes
                .iter()
                .filter(|rune| matches!(rune, Rune::Iron))
                .collect::<Vec<_>>()
                .len() as u16,
        );

        // let qual = 1.0 + self.quality.0 as f32 / 100.;
        // let flat = (base_phys.range + flat_phys_explicit.range) as f32;
        // let percent = 1.0 + (phys_modifier_explicit.0 + runes_phys_modifier.0) as f32 / 100.;

        // println!("{qual} {flat} {percent}");

        (1.0 + self.quality.0 as f32 / 100.0)
            * (base_phys.range + flat_phys_explicit.range) as f32
            * (self.attack_speed * (1.0 + attack_speed_modifier.0 as f32 / 100.))
            * (1.0 + (phys_modifier_explicit.0 + runes_phys_modifier.0) as f32 / 100.)
            * 0.5
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Explicit {
    Flat(FlatDamage),
    Phys(PhysModifier),
    AtkSpd(AttackSpeedModifier),
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AttackSpeedModifier(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhysModifier(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Quality(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct FlatDamage {
    pub damage_type: DamageType,
    pub range: Range,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum DamageType {
    #[default]
    #[serde(rename = "physical")]
    Physical,
    #[serde(rename = "fire")]
    Fire,
    #[serde(rename = "cold")]
    Cold,
    #[serde(rename = "lightning")]
    Lightning,
    #[serde(rename = "chaos")]
    Chaos,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phys_dps() {
        let white_weapon = Weapon {
            base_damage: vec![FlatDamage {
                damage_type: DamageType::Physical,
                range: Range(41, 76),
            }],
            quality: Quality(20),
            attack_speed: 1.25,
            explicits: vec![],
            runes: vec![],
        };

        assert!((87.75 - white_weapon.phys_dps()).abs() < 0.00001)
    }
}
