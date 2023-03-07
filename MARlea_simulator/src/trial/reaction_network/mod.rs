pub mod reaction; 

use std::{collections::{HashSet, HashMap}};
use rand::Rng;
use reaction::{Reaction, term::species::Species};

/// Contains a set of all the declared reactions, as well as a set of all of the available species.
#[derive(Clone)]
pub struct ReactionNetwork <'reacting> {
    reactions: HashSet<Reaction<'reacting>>,
    null_adjacent_reactions: HashSet<&'reacting Reaction<'reacting>>,
    solution: HashMap<&'reacting Species, Species>,
}

impl<'reacting> ReactionNetwork <'reacting> {

    pub fn get_null_adjacent_reactions(&self) -> HashSet<&Reaction> {
        return self.null_adjacent_reactions.clone();
    }

    /// itterates through all reactions and finds reactions with no reactants it then inserts them into the null_adjacent_reactions set. 
    /// after this it gets the species from that reaction's products and searches for reactions which use those species as their reactants. 
    /// 
    /// this has awful runtime but should only be called once per execution of the program 
    fn gen_null_adjacent_reactions(&'reacting mut self) {

        self.null_adjacent_reactions.clear();

        for reaction in &self.reactions {

            if reaction.get_reactants().is_empty() {

                if self.null_adjacent_reactions.insert(reaction) {
                    for product in reaction.get_products() {
                        let null_generated_species = product.get_species();

                        for secondary_reaction in &self.reactions {
                            for secondary_reactant in secondary_reaction.get_reactants() {

                                if *null_generated_species == *secondary_reactant.get_species() {
                                    if !self.null_adjacent_reactions.insert(secondary_reaction) {
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

        return;
    }

    /// Returns the subset of local reactions set is possible based on the Species.count values in solution
    pub fn get_possible_reactions<'getting_reactions> (&'getting_reactions self) -> HashSet<&Reaction<'reacting>> {

        let mut possible_reactions = HashSet::new();

        for reaction in &self.reactions {
            if reaction.is_possible() {
                possible_reactions.insert(reaction);
            }
        }

        return possible_reactions;
    }

    /// takes outputs the total sum of the reaction rates in the provided hash set of reactions
    fn sum_reaction_rates (&self, reactions: &HashSet<&Reaction<'reacting>>) -> u64 {
        let mut sum: u64 = 0; 
        for reaction in reactions {
            sum += reaction.get_reaction_rate();
        }
        return sum;
    }

    /// Randomly selects a reaction from the provided set using a probablility generated from the total number of reactions with reach one scaled by reaction rate,
    pub fn get_next_reaction(&self, possible_reactions: &HashSet<&'reacting Reaction<'reacting>>) -> Option<&'reacting Reaction<'reacting>> {
    
        let mut index = rand::thread_rng().gen_range(0.. self.sum_reaction_rates(possible_reactions) + 1);
        let mut next_reaction: Option<&'reacting Reaction<'reacting>>= None;

        // if loop finishes before index < reaction rate then the return value will be null 
        for reaction in possible_reactions {
            if reaction.get_reaction_rate() > index {
                next_reaction = Some(reaction);
                break;
            }
            else {index -= reaction.get_reaction_rate();}
        }

        return next_reaction;
    }

    /// Updates local solution with the effects of a given reaction
    pub fn react<'reaction> (&'reaction mut self, reaction: &'reaction Reaction<'reacting>) {
        
        // this could maybe be done by using dedicated functions in Term struct which would allow for storage of species in a hash set
        for reactant in reaction.get_reactants() {
            self.solution.entry(reactant.get_species())
            .and_modify(|species: &mut Species| species.set_count(species.get_count() - reactant.get_coefficient()));
        }

        for product in reaction.get_products() {
            self.solution.entry(product.get_species())
            .and_modify(|species: &mut Species| species.set_count(species.get_count() + product.get_coefficient()));
        }

        return;
    }

    pub fn get_solution (&self) -> &HashMap<&Species, Species> {
        return &self.solution;
    }

}
