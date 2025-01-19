use parser::Parsed;
use serde::{Deserialize, Serialize};
#[allow(unused)]
use weapon::{AttackSpeedModifier, DamageType, Explicit, FlatDamage, PhysModifier, Quality, Range};

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
    //
}
