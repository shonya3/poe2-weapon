#![allow(unused)]
pub mod bases;

use serde::{Deserialize, Serialize};
use weapon::{Explicit, FlatDamage, Quality, Rune};

pub fn parse(clipboard_input: &str) -> Parsed {
    todo!()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parsed {
    pub base: String,
    pub explicits: Vec<Explicit>,
    pub runes: Vec<Rune>,
    pub quality: Quality,
}
