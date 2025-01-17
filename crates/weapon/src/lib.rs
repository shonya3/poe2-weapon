use std::ops::Add;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Range(pub u16, pub u16);

impl Add for Range {
    type Output = u16;

    fn add(self, rhs: Self) -> Self::Output {
        self.0 + self.1 + rhs.0 + rhs.1
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Rune {
    Iron,
}

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
            Explicit::FlatDamage(flat_damage) => {
                if matches!(flat_damage.damage_type, DamageType::Physical) {
                    flat_phys_explicit = *flat_damage
                }
            }
            Explicit::PercentPhys(phys) => phys_modifier_explicit = *phys,
            Explicit::AttackSpeed(attack_speed) => attack_speed_modifier = *attack_speed,
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
        // let flat = (base_phys.value + flat_phys_explicit.value) as f32;
        // let percent = 1.0 + (phys_modifier_explicit.0 + runes_phys_modifier.0) as f32 / 100.;

        // println!("{qual} {flat} {percent}");

        (1.0 + self.quality.0 as f32 / 100.0)
            * (base_phys.value + flat_phys_explicit.value) as f32
            * (self.attack_speed * (1.0 + attack_speed_modifier.0 as f32 / 100.))
            * (1.0 + (phys_modifier_explicit.0 + runes_phys_modifier.0) as f32 / 100.)
            * 0.5
    }
}

#[derive(Debug, Clone)]
pub enum Explicit {
    FlatDamage(FlatDamage),
    PercentPhys(PhysModifier),
    AttackSpeed(AttackSpeedModifier),
    Other(String),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AttackSpeedModifier(pub u8);

#[derive(Debug, Clone, Copy, Default)]
pub struct PhysModifier(pub u16);

#[derive(Debug, Clone, Copy, Default)]
pub struct Quality(pub u8);

#[derive(Debug, Clone, Copy, Default)]
pub struct FlatDamage {
    pub damage_type: DamageType,
    pub value: Range,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum DamageType {
    #[default]
    Physical,
    Fire,
    Cold,
    Lightning,
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
                value: Range(41, 76),
            }],
            quality: Quality(20),
            attack_speed: 1.25,
            explicits: vec![],
            runes: vec![],
        };

        assert!((87.75 - white_weapon.phys_dps()).abs() < 0.00001)
    }
}
