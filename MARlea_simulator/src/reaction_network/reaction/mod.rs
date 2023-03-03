use std::{collections::{HashSet}, hash::{Hash, Hasher}};
use std::collections::hash_map::DefaultHasher;
use term::Term;

pub mod term;

/// Stores a set for the reaction reactants and products. 
/// Eeach element contains the variable key used by a Solution struct as well as a reaction rate. 
/// This struct should only be used inside of the Reaction_Network Struct 
#[derive(Eq, PartialEq,Clone)]
pub struct Reaction <'reaction> {
    reactants: HashSet<Term<'reaction>>,
    products: HashSet<Term<'reaction>>,
    reaction_rate: u32,
}

impl<'reaction> Reaction <'reaction> {

    /// returns a reference to the reactants set within a reaction
    pub fn get_reactants(&self) -> &HashSet<Term<'reaction>> {
        return &self.reactants;
    }

    /// returns a reference to the products set within a reaction
    pub fn get_products(&self) -> &HashSet<Term<'reaction>> {
        return &self.products;
    }

    /// returns the reaction rate
    pub fn get_reaction_rate (&self) -> u64 {
        return self.reaction_rate as u64;
    }

    pub fn is_possible (&self) -> bool {
        let mut reaction_possible = true;

        for reactant in &self.reactants {
            if reactant.get_coefficient() > reactant.get_species().get_count() {
                reaction_possible = false;
                break;
            }
        }
        
        return reaction_possible;
    }
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

