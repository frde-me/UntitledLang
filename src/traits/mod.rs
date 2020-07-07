use serde::{Serialize, Deserialize};
use crate::loader::{HasKey, Load};
use crate::phonetics::PositionedPhonetic;
use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Deserialize, Serialize, Debug)]
pub struct Trait {
    name: TraitName,
    variants: Vec<TraitVariant>
}

identifier!(Trait, name, TraitName);

loader!(Trait, TraitName, "res/traits");

#[derive(Deserialize, Serialize, Debug)]
pub struct TraitVariant {
    name: TraitVariantName,
    phonetic: PositionedPhonetic
}

identifier!(TraitVariant, name, TraitVariantName);