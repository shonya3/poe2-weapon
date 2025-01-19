#![allow(unused)]
pub mod bases;

use bases::BASES;
use serde::{Deserialize, Serialize};
use weapon::{Explicit, FlatDamage, Quality, Rune};

pub fn parse(text: &str) -> Parsed {
    text.lines().take(5).find_map(|s| {
        let s = s.trim();
        if BASES.contains(&s) {
            return Some(s.to_owned());
        }

        None
    });

    todo!()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parsed {
    pub base: String,
    pub explicits: Vec<Explicit>,
    pub runes: Vec<Rune>,
    pub quality: Quality,
}
