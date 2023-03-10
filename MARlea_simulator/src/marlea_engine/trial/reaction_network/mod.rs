pub mod reaction; 

use std::{collections::{HashSet, HashMap}};
use rand::Rng;
use reaction::{Reaction, term::species::Species};


#[derive(Clone)]
/// A `ReactionNetwork` represents a computational netowork of chemical reactions.
///
/// It contains four main components:
///
/// - `reactions`: a set of all the reactions in the network, represented as instances of `Reaction`.
/// - `possible_reactions`: a subset of `reactions` that are currently possible to occur based on the current state
///                        of the system (i.e. the concentration of Species in solution). This is updated at each time step.
/// - `null_adjacent_reactions`: a subset of `reactions` that involve only products, 
///                              or involve reactants produced by `reactions` only involving products.
///                              i.e. they are adjacent to null species.
///                              This is used to speed up computations.
/// - `solution`: a dictionary that maps references to species to their current value in the system
///
/// The lifetime parameter `'reaction_network` is used to tie the struct to the lifetime of its dependencies,
/// such as instances of `Reaction` and `Species`.
pub struct ReactionNetwork <'reaction_network> where 'reaction_network: {
    reactions: HashSet<Reaction<'reaction_network>>,
    possible_reactions: HashSet<Reaction<'reaction_network>>, 
    null_adjacent_reactions: HashSet<Reaction<'reaction_network>>,
    solution: HashMap<&'reaction_network Species, Species>,
}

impl<'reaction_netowrk, 'reaction> ReactionNetwork <'reaction_netowrk> where 'reaction_netowrk:'reaction {

    pub fn new(reactions: HashSet<Reaction<'reaction_netowrk>>, solution: HashMap<&'reaction_netowrk Species, Species>)-> Self {
        // Initialize null_adjacent_reactions and possible_reactions as empty HashSet
        let null_adjacent_reactions = HashSet::new();
        let possible_reactions = HashSet::new();

        // Return a new instance of Self with the provided arguments and initialized fields.
        Self { reactions, solution, null_adjacent_reactions, possible_reactions }
    }

    pub fn get_null_adjacent_reactions(&self) -> &HashSet<Reaction> {
        // Returns a reference to the null_adjacent_reactions HashSet
        return &self.null_adjacent_reactions;
    }

    // Clears the null_adjacent_reactions HashSet and generates a new set.
    fn gen_null_adjacent_reactions(&'reaction_netowrk mut self) {

        self.null_adjacent_reactions.clear();

        for reaction in &self.reactions {
            // Check for reactions that only have products (null adjacent).
            if reaction.get_reactants().is_empty() {

                // Insert the reaction into the null_adjacent_reactions HashSet and access its corresponding product(s)
                if self.null_adjacent_reactions.insert(reaction.clone()) {
                    for product in reaction.get_products() {
                        let null_generated_species = product.get_species();

                        // For each secondary reaction, check if its reactant species matches the current null generated species
                        for secondary_reaction in &self.reactions {
                            for secondary_reactant in secondary_reaction.get_reactants() {

                                if *null_generated_species == *secondary_reactant.get_species() {
                                    // Insert the reaction into the null_adjacent_reactions HashSet.
                                    // Fail if we cannot insert the reaction into the hashset.
                                    if !self.null_adjacent_reactions.insert(secondary_reaction.clone()) {
                                        panic!("failed to insert into null_adjacent_reactions");
                                    }
                                }
                            }
                        }
                    }
                }
                else {
                    panic!("failed to insert into null_adjacent_reactions");
                }
            }
        }
    }


    pub fn get_possible_reactions<'getting> (&'getting self) -> &'getting HashSet<Reaction<'reaction_netowrk>> 
        where 'reaction:'getting{
        return &self.possible_reactions;
    }

    fn find_possible_reactions<'finding>(&'finding mut self) {
        self.possible_reactions.clear();
        
        // loop over all reactions and check if it's possible for them to occur based on current species concentration
        for reaction in &self.reactions {
            if reaction.is_possible() {
                self.possible_reactions.insert(reaction.clone()); // add reaction to list of possible reactions
            }
        }
    }

    fn sum_reaction_rates (&self) -> u64 {
        let mut sum: u64 = 0; 
        // loop over all possible reactions and sum their reaction rates
        for reaction in &self.possible_reactions {
            sum += reaction.get_reaction_rate();
        }
        return sum;
    }


// Get a possible reaction from the set of possible reactions with weighted probability
pub fn get_next_reaction<'getting> (&'getting self) -> Option<Reaction<'reaction>> where 'reaction:'getting {
    let mut index = rand::thread_rng().gen_range(0.. self.sum_reaction_rates() + 1);
    let mut next_reaction: Option<Reaction<'reaction>>= None;

    // iterate through all possible valid reactions and pick one based on its probability 
    for reaction in self.get_possible_reactions() {
        if reaction.get_reaction_rate() > index {
            next_reaction = Some(reaction.clone());
            break;
        } else {
            index -= reaction.get_reaction_rate();
        }
    }

    return next_reaction;
}

// This function reacts based on the randomly selected Reaction instance
pub fn react<'reacting> (&'reacting mut self)
where
    'reaction_netowrk:'reacting,
    'reacting:'reaction
{
    // update the list of possible reactions. 
    self.find_possible_reactions();

    // if the system has a next reaction, modify the solutions containing reactants and products 
    if let Some(reaction) = self.get_next_reaction() {
        for reactant in reaction.get_reactants() {
            self.solution.entry(reactant.get_species())
                .and_modify(|species: &mut Species| species.set_count(species.get_count() - reactant.get_coefficient()));
        }

        for product in reaction.get_products() {
            self.solution.entry(product.get_species())
                .and_modify(|species: &mut Species| species.set_count(species.get_count() + product.get_coefficient()));
        }
    } 
    // else, panic with an error message 
    else {
        panic!("failed to get next reaction in react()");
    }
}

// returns a reference to the map containing the current state of the reaction network 
pub fn get_solution (&'reaction_netowrk self) -> &HashMap<&Species, Species> {
    return &self.solution;
}


}
