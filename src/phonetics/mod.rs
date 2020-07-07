use crate::loader::{HasKey, Load};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Phonetic component is a sound and a position
///
/// It can be either a `Word`, `Prefix` or `Suffix`
/// The sound is represented in the `International Phonetic Alphabet` or `IPA`
#[derive(Deserialize, Serialize, Debug)]
pub struct PositionedPhonetic {
    sound: IPA,
    word_position: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Phonetic {
    ipa: IPA,
}

identifier!(Phonetic, ipa, IPA);
loader!(Phonetic, IPA, "res/phonetics");
