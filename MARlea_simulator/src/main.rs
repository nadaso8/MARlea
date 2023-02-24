use std::{collections::{HashMap, HashSet}};


/// Contains a Name and value representing the count of some named DNA string
enum Species {
    Name(String),
    Value(u64),
}

/// Contains the data for a single term within a larger reaction.
/// Species is a reference to a named value in solution which will be added to or subtracted from. 
/// Coefficient is the value to add or subtract
enum Term {
    Name(String),// Could this be done using a pointer to some species enum instead 
    Coefficient(u8),
}

/// Stores a set for the reaction reactants and products. 
/// Eeach element contains the variable key used by a Solution struct as well as a reaction rate. 
/// This struct should only be used inside of the Reaction_Network Struct 
struct Reaction {
    reactants: HashSet<Term>,
    products: HashSet<Term>,
    reaction_rate: u32,
}

impl Reaction {
    /// simulates the effects of a reaction occuring
    /// <reaction> - specifies a reaction struct containing the reactants to be removed and products to be added to solution
    /// <solution> - specifies the solution datastructure to subtract and add values to
    fn react(&self, &mut solution: HashMap<String,u64>) {
        
        // subtracts reactant coefficients from values in solution in solution and removes the solution entry if it would be 0
        for current_reactant in &self.reactants {
            if solution.get(current_reactant) == u64::from(current_reactant) { solution.remove(current_reactant.0);}
            solution.entry(*current_reactant)
            .and_modify(|solution_value| *solution_value -= *current_reactant.1 as u64);
            
        }
        
        // adds product coefficients to values in solution solution in solution or creates a new solution entry if one was not already present
        for product in &self.products {
            solution.entry(product.0)
            .and_modify(|solution_value| *solution_value += product.1 as u64)
            .or_insert(product.1 as u64); // .or_insert() might not be needed if possible variables are inserted before the react function runs
        }
    }   
}
/// Contains a set of all the declared reactions, as well as a set of all the reactions which may react next.
/// possible_next_reaction is determined by comparing the coefficients for a given reactant to the current value held for that name in strands
struct ReactionNework {
    reactions: HashSet<Reaction>,
    possible_next_reactions: HashSet<Reaction>,// possibly not needed
    solution: HashMap<Species>,
}

impl ReactionNework {
    /// Randomly selects a reaction from possible_next_reactions based on the probability in reaction rate,
    /// then reacts it updating the local strands values
    fn get_next_reaction () {

    }

    ///
    fn possible() {

    }
}

fn main () {

}