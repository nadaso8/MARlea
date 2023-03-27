pub mod term;

use std::{collections::{HashSet, HashMap}, hash::{Hash, Hasher}};
use std::collections::hash_map::DefaultHasher;
use term::{Term, solution::Species};

/// Stores a set for the reaction reactants and products. 
/// Eeach element contains the variable key used by a Solution struct as well as a reaction rate. 
/// This struct should only be used inside of the Reaction_Network Struct 
#[derive(Debug, Eq, PartialEq,Clone)]
pub struct Reaction {
    reactants: HashSet<Term>,
    products: HashSet<Term>,
    reaction_rate: u64,
}

impl Reaction {

    pub fn new (reactants: HashSet<Term>, products: HashSet<Term>, reaction_rate: u64) -> Self {
        return Self { reactants: reactants, products: products, reaction_rate: reaction_rate};
    }
    
    /// returns a reference to the reactants set within a reaction
    pub fn get_reactants(&self) -> &HashSet<Term> {
        return &self.reactants;
    }

    /// returns a reference to the products set within a reaction
    pub fn get_products(&self) -> &HashSet<Term> {
        return &self.products;
    }

    /// returns the reaction rate
    pub fn get_reaction_rate (&self) -> u128 {
        return self.reaction_rate as u128;
    }

    pub fn is_possible (&self, solution: &HashMap<Species,Species>) -> bool {
        let mut reaction_possible = true;

        for reactant in &self.reactants {
            if let Some(Species::Count(current_count)) = solution.get(&reactant.get_species_name()) {
                if reactant.get_coefficient() > *current_count {
                    reaction_possible = false;
                    break;
                }
            }
        }
        
        return reaction_possible;
    }
}

impl Hash for Reaction {
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