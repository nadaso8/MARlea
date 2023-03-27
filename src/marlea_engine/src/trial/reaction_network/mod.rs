use std::collections::HashSet;
use rand::Rng;
use reaction::{Reaction, term::solution::{Species, Solution}};

pub mod reaction; 

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
/// - `solution`: a dictionary that maps Species::Names to their Species::counts
///
/// The lifetime parameter `'reaction_network` is used to tie the struct to the lifetime of its dependencies,
/// such as instances of `Reaction` and `Species`.
pub struct ReactionNetwork {
    reactions: HashSet<Reaction>,
    possible_reactions: HashSet<Reaction>, 
    null_adjacent_reactions: HashSet<Reaction>,
    solution: Solution,
}

impl ReactionNetwork {

    pub fn new(reactions: HashSet<Reaction>, solution: Solution)-> Self {
        // Initialize null_adjacent_reactions and possible_reactions as empty HashSet
        let null_adjacent_reactions = HashSet::new();
        let possible_reactions = HashSet::new();

        // Make a new instance of Self with the provided arguments and initialized fields.
        let mut new_netowrk = Self{reactions, solution, null_adjacent_reactions, possible_reactions};

        // Generate and cache null adjacent reactions up front
        new_netowrk.gen_null_adjacent_reactions();

        return new_netowrk;
    }

    pub fn get_null_adjacent_reactions(&self) -> &HashSet<Reaction> {
        // Returns a reference to the null_adjacent_reactions HashSet
        return &self.null_adjacent_reactions;
    }

    // Clears the null_adjacent_reactions HashSet and generates a new set.
    fn gen_null_adjacent_reactions(&mut self) {

        self.null_adjacent_reactions.clear();

        for reaction in &self.reactions {
            // Check for reactions that only have products (null adjacent).
            if reaction.get_reactants().is_empty() {

                // Insert the reaction into the null_adjacent_reactions HashSet and access its corresponding product(s)
                if self.null_adjacent_reactions.insert(reaction.clone()) {
                    for product in reaction.get_products() {
                        let null_generated_species = product.get_species_name();

                        // For each secondary reaction, check if its reactant species matches the current null generated species
                        for secondary_reaction in &self.reactions {
                            for secondary_reactant in secondary_reaction.get_reactants() {

                                if null_generated_species == secondary_reactant.get_species_name() {
                                    // Insert the reaction into the null_adjacent_reactions HashSet.
                                    self.null_adjacent_reactions.insert(secondary_reaction.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }


    pub fn get_possible_reactions(&self) -> &HashSet<Reaction> {
        return &self.possible_reactions;
    }

    fn find_possible_reactions<'finding>(&'finding mut self) {
        self.possible_reactions.clear();
        
        // loop over all reactions and check if it's possible for them to occur based on current species concentration
        for reaction in &self.reactions {
            if reaction.is_possible(&self.solution.species_counts) {
                self.possible_reactions.insert(reaction.clone()); // add reaction to list of possible reactions
            }
        }
    }

    fn sum_reaction_rates (&self) -> u128 {
        let mut sum: u128 = 0; 
        // loop over all possible reactions and sum their reaction rates
        for reaction in &self.possible_reactions {
            sum += reaction.get_reaction_rate();
        }
        return sum;
    }


    // Get a possible reaction from the set of possible reactions with weighted probability
    pub fn get_next_reaction<'getting> (&'getting self) -> Option<Reaction> {
        let mut index = rand::thread_rng().gen_range(0.. self.sum_reaction_rates());
        let mut next_reaction: Option<Reaction>= None;

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
    pub fn react<'reacting> (&'reacting mut self) {
        // update the list of possible reactions. 
        self.find_possible_reactions();

        if !self.possible_reactions.is_empty() {
            if let Some(reaction) = self.get_next_reaction() {
                for reactant in reaction.get_reactants() {
                    self.solution.species_counts.entry(reactant.get_species_name().clone())
                        .and_modify(|species_count|
                            if let Species::Count(current_count) =species_count{
                                *current_count -= reactant.get_coefficient();
                            });
                }

                for product in reaction.get_products() {
                    self.solution.species_counts.entry(product.get_species_name().clone())
                        .and_modify(|species_count|
                            if let Species::Count(current_count) = species_count {
                                *current_count += product.get_coefficient();
                            });
                }
            } 
            else {
                panic!("failed to get next reaction in react()");
            }
        }
    }

    // returns a reference to the map containing the current state of the reaction network 
    pub fn get_solution(&self) -> &Solution {
        return &self.solution;
    }
}

