use serde::{Serialize, Deserialize};
use crate::loader::{HasKey, Load};
use std::collections::HashMap;
use lazy_static::lazy_static;

/// A Phonetic component is a sound and a position
///
/// It can be either a `Word`, `Prefix` or `Suffix`
/// The sound is represented in the `International Phonetic Alphabet` or `IPA`
#[derive(Deserialize, Serialize, Debug)]
pub struct PositionedPhonetic {
    sound: IPA,
    word_position: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Phonetic {
    ipa: IPA
}

identifier!(Phonetic, ipa, IPA);
loader!(Phonetic, IPA , "res/phonetics");
