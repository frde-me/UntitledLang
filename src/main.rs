#![feature(associated_type_defaults)]

use crate::concept::{Concept, ConceptName};
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

    let name: ConceptName = "person".into();
    let person: &Concept = name.load().unwrap();
    let sound: &Phonetic = person.phonetic.sound.load().unwrap();
    println!("person: {:?}", person);
    println!("sound: {:?}", sound);

    let traits = loader::load_from_folder::<TraitName, Trait>("res/traits").unwrap();
    println!("{:#?}", traits);

    let phonetics = loader::load_from_folder::<IPA, Phonetic>("res/phonetics").unwrap();
    println!("{:#?}", phonetics);
}
