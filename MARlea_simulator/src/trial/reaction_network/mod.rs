pub mod reaction; 

use std::{collections::{HashSet, HashMap}};
use rand::Rng;
use reaction::{Reaction, term::species::Species};

/// Contains a set of all the declared reactions, as well as a set of all of the available species.
#[derive(Clone)]
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

    /// itterates through all reactions and finds reactions with no reactants it then inserts them into the null_adjacent_reactions set. 
    /// after this it gets the species from that reaction's products and searches for reactions which use those species as their reactants. 
    /// 
    /// this has awful runtime but should only be called once per execution of the program 
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

    /// Returns the subset of local reactions set is possible based on the Species.count values in solution
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

    /// takes outputs the total sum of the reaction rates in the provided hash set of reactions
    fn sum_reaction_rates (&self) -> u64 {
        let mut sum: u64 = 0; 
        for reaction in &self.possible_reactions {
            sum += reaction.get_reaction_rate();
        }
        return sum;
    }

    /// Randomly selects a reaction from the provided set using a probablility generated from the total number of reactions with reach one scaled by reaction rate,
    pub fn get_next_reaction<'getting> (&'getting self) -> Option<Reaction<'reaction>> where 'reaction:'getting {
    
        let mut index = rand::thread_rng().gen_range(0.. self.sum_reaction_rates() + 1);
        let mut next_reaction: Option<Reaction<'reaction>>= None;

        // if loop finishes before index < reaction rate then the return value will be null 
        for reaction in self.get_possible_reactions() {
            if reaction.get_reaction_rate() > index {
                next_reaction = Some(reaction.clone());
                break;
            }
            else {index -= reaction.get_reaction_rate();}
        }

        return next_reaction;
    }

    /// Updates local solution with the effects of a given reaction
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
