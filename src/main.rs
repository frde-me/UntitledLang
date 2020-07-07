#![feature(associated_type_defaults)]

use crate::concept::{Concept, ConceptName};
use crate::loader::Load;
use crate::phonetics::{Phonetic, IPA};
use crate::traits::{Trait, TraitName};

#[macro_use]
mod loader;
mod concept;
mod phonetics;
mod traits;

fn main() {
    let concepts = loader::load_from_folder::<ConceptName, Concept>("res/concepts").unwrap();
    println!("{:#?}", concepts);

    println!("person: {:?}", Concept::load("person"));

    let traits = loader::load_from_folder::<TraitName, Trait>("res/traits").unwrap();
    println!("{:#?}", traits);

    let phonetics = loader::load_from_folder::<IPA, Phonetic>("res/phonetics").unwrap();
    println!("{:#?}", phonetics);
}
