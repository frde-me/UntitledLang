use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::loader::{HasKey, Load};
use crate::phonetics::PositionedPhonetic;

#[derive(Deserialize, Serialize, Debug)]
pub struct Trait {
    name: TraitName,
    variants: Vec<TraitVariant>,
}

key!(Trait, name, TraitName, LOAD);
loader!(Trait, TraitName, "res/traits");

#[derive(Deserialize, Serialize, Debug)]
pub struct TraitVariant {
    name: TraitVariantName,
    phonetic: PositionedPhonetic,
}

key!(TraitVariant, name, TraitVariantName);
