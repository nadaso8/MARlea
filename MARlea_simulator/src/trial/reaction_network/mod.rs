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

    pub fn get_null_adjacent_reactions(&self) -> &HashSet<Reaction> {
        return &self.null_adjacent_reactions;
    }


    fn gen_null_adjacent_reactions(&'reaction_netowrk mut self) {

        self.null_adjacent_reactions.clear();

        for reaction in &self.reactions {

            if reaction.get_reactants().is_empty() {

                if self.null_adjacent_reactions.insert(reaction.clone()) {
                    for product in reaction.get_products() {
                        let null_generated_species = product.get_species();

                        for secondary_reaction in &self.reactions {
                            for secondary_reactant in secondary_reaction.get_reactants() {

                                if *null_generated_species == *secondary_reactant.get_species() {
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

    pub fn get_possible_reactions<'getting> (&'getting self) -> &'getting HashSet<Reaction<'reaction_netowrk>> where 'reaction:'getting{
        return &self.possible_reactions;
    }

    fn find_possible_reactions<'finding>(&'finding mut self) {
        self.possible_reactions.clear();

        for reaction in &self.reactions {
            if reaction.is_possible() {
                self.possible_reactions.insert(reaction.clone());
            }
        }
    }

    fn sum_reaction_rates (&self) -> u64 {
        let mut sum: u64 = 0; 
        for reaction in &self.possible_reactions {
            sum += reaction.get_reaction_rate();
        }
        return sum;
    }

    pub fn get_next_reaction<'getting> (&'getting self) -> Option<Reaction<'reaction>> where 'reaction:'getting {
    
        let mut index = rand::thread_rng().gen_range(0.. self.sum_reaction_rates() + 1);
        let mut next_reaction: Option<Reaction<'reaction>>= None;

        for reaction in self.get_possible_reactions() {
            if reaction.get_reaction_rate() > index {
                next_reaction = Some(reaction.clone());
                break;
            }
            else {index -= reaction.get_reaction_rate();}
        }

        return next_reaction;
    }

    pub fn react<'reacting> (&'reacting mut self)

    where
    'reaction_netowrk:'reacting,
    'reacting:'reaction

    {

        self.find_possible_reactions();

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
        else {panic!("failed to get next reaction in react()");}
    }

    pub fn get_solution (&'reaction_netowrk self) -> &HashMap<&Species, Species> {
        return &self.solution;
    }

}
