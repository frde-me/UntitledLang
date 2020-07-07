use serde::{Deserialize, Serialize};
use crate::loader::{HasKey, Load};
use crate::phonetics::PositionedPhonetic;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::borrow::Borrow;
use crate::identifier;

/// A concept is an base `Object` in the world that can be represented on it's own and is impactful.
/// It does not have to be physical.
///
/// Ex: `Being`, `Universe`, ...
///
/// Note that we can define a concept, like `Human`, that is a modification, of another concept, `Being`, as a convenience.
/// The concepts should be far enough apart and impactful. For instance `Child` should be a `Young Human`, it does not need it's own base concept
#[derive(Serialize, Deserialize, Debug)]
pub struct Concept {
    name: ConceptName,
    #[serde(default)]
    is: Vec<String>,
    phonetic: PositionedPhonetic
}

identifier!(Concept, name, ConceptName);

loader!(Concept, ConceptName, "res/concepts");
