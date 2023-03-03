pub mod reaction;

use std::{collections::{HashSet, HashMap}};
use reaction::{Reaction, term::{Term}, term::species::{Species}};

/// Contains a set of all the declared reactions, as well as a set of all of the available species.
struct ReactionNetwork <'reacting> {
    reactions: HashSet<Reaction<'reacting>>,
    solution: HashMap<&'reacting Species, Species>,
}

impl<'reacting> ReactionNetwork <'reacting> {

    /// Returns the subset of local reactions set is possible based on the Species.count values in solution
    fn get_possible_reactions (&self) -> HashSet<Reaction<'reacting>> {
        
        let mut possible_reactions: HashSet<Reaction<'reacting>> = HashSet::new();

            for reaction in &self.reactions {

                let mut reaction_possible = true;

                for reactant in &reaction.reactants {

                    if reactant.coefficient as u64 > reactant.species.count {
                        reaction_possible = false;
                        break;
                    }

                }

                if reaction_possible {
                    if possible_reactions.insert(reaction.clone()) { }
                }
            }
        return possible_reactions
    }

    /// Randomly selects a reaction from the provided set using a probablility generated from the total number of reactions with reach one scaled by reaction rate,
    fn get_next_reaction(possible_reactions: HashSet<Reaction<'reacting>>) -> &'reacting Reaction<'reacting> {
        let mut next_reaction: &Reaction = todo!();
    
        
        return next_reaction;
    }

    /// Updates local solution with the effects of a given reaction
    fn react(& mut self, reaction: &'reacting Reaction<'reacting>) {
        
        for reactant in &reaction.reactants {
            self.solution.entry(reactant.species)
            .and_modify(|species| species.count -= u64::from(reactant.coefficient));
        }

        for product in &reaction.products {
            self.solution.entry(product.species)
            .and_modify(|species| species.count -= u64::from(product.coefficient));
        }
    }
}
