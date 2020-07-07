use crate::loader::{HasKey, Load};
use crate::phonetics::PositionedPhonetic;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Trait {
    name: TraitName,
    variants: Vec<TraitVariant>,
}

key!(Trait, name, TraitName);

loader!(Trait, TraitName, "res/traits");

#[derive(Deserialize, Serialize, Debug)]
pub struct TraitVariant {
    name: TraitVariantName,
    phonetic: PositionedPhonetic,
}

key!(TraitVariant, name, TraitVariantName);
