pub mod reaction_network;
pub mod stability_checker;

use std::collections::hash_map::HashMap;
use reaction_network::{ReactionNetwork, reaction::term::species::Species};
use stability_checker::NetworkStability;

struct Simulation <'getting_result> {
    reaction_network: ReactionNetwork<'getting_result>,
    simulation_state: NetworkStability<'getting_result>,
}

impl<'getting_result> Simulation<'getting_result> {
    pub fn simulate (&mut self) -> HashMap<&Species, Species> {
        loop {
            match &self.simulation_state {

                NetworkStability::Initial => {
                    let possible_reactions = self.reaction_network.get_possible_reactions();
                    self.simulation_state.update(&possible_reactions);
                }
                
                NetworkStability::Unstable(set) | NetworkStability::SemiStable(set, _) => {
                    self.reaction_network.react(self.reaction_network.get_next_reaction(&set).expect("Failed to get next reaction while reacting"));
                    let possible_reactions = self.reaction_network.get_possible_reactions();
                    self.simulation_state.update(&possible_reactions);
                }
                
                NetworkStability::Stable => {
                    return self.reaction_network.get_solution().clone();
                }

            }
        }
    }
}