use std::{collections::{HashSet}, hash::{Hash, Hasher}};
use std::collections::hash_map::DefaultHasher;
use term::Term;

pub mod term;

/// Stores a set for the reaction reactants and products. 
/// Eeach element contains the variable key used by a Solution struct as well as a reaction rate. 
/// This struct should only be used inside of the Reaction_Network Struct 
#[derive(Eq, PartialEq,Clone)]
pub struct Reaction <'reaction> {
    pub reactants: HashSet<Term<'reaction>>,
    pub products: HashSet<Term<'reaction>>,
    reaction_rate: u32,
}

impl<'reaction> Hash for Reaction<'reaction> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hasher = DefaultHasher::new();
        for term in &self.reactants {
            term.hash(&mut hasher);
        }
        for term in &self.products {
            term.hash(&mut hasher);
        }
        self.reaction_rate.hash(&mut hasher);
        hasher.finish().hash(state);
    }
}

