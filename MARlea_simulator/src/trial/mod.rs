// Import necessary modules 
pub mod reaction_network; // Module that holds our ReactionNetwork details
use std::collections::HashMap; // Hashmap class used collectively within the module
use reaction_network::{ReactionNetwork, reaction::term::species::Species}; // Import relevant classes from ReactionNetwork module

const MAX_SEMI_STABLE_CYCLES: i32 = 99; // maximum number of cycles before a semi-stable network is marked as stable

pub struct Trial<'trial> {
    reaction_network: ReactionNetwork<'trial>, // Instance of Reaction Network for this trial
    stability: Stability, // Current stability status of the trial/experiment (initially set to "Initial")
}


impl<'trial> Trial<'trial> {

    /// Generates a new instance of `Trial` by taking in an instance of `ReactionNetwork`.
    pub fn from(reaction_network: ReactionNetwork<'trial>) -> Self {
        Self {
            reaction_network,
            stability: Stability::Initial,
        }
    }

    /// This function simulates a Reaction Network by running reactions on it until the network
    /// reaches a stable state. It takes a mutable reference to a Trial instance and returns a 
    /// reference to a HashMap containing the Species instances in the stable network solution.
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference to a Trial instance
    ///
    /// # Returns
    ///
    /// A reference to a HashMap that contains all the `species` after simulation keyd by their references.
    pub fn simulate(&mut self) -> &HashMap<&Species,Species> {
        loop{

            self.step();

             // If a stable state has been reached, return the current network solution
            if let Stability::Stable = self.stability {
                return self.reaction_network.get_solution();
            }

        }
    }

    /// Handles progressing the simulation one step further by evaluating the current status of the system and performing reactions if necessary.
    fn  step (&mut self) {
        match self.stability {
            Stability::Initial => {
                 
                if self.reaction_network.get_possible_reactions().is_empty() {
                    self.stability = Stability::Stable;
                }

                else if self.reaction_network.get_possible_reactions().is_subset(&self.reaction_network.get_null_adjacent_reactions()) {
                    self.stability = Stability::SemiStable(0);
                } else {
                    
                    self.stability = Stability::Unstable;
                }
            } 
            Stability::Unstable => {

                if self.reaction_network.get_possible_reactions().is_empty() {
                    self.stability = Stability::Stable;
                }
                else if self.reaction_network.get_possible_reactions().is_subset(self.reaction_network.get_null_adjacent_reactions()) {
                    self.reaction_network.react();
                    self.stability = Stability::SemiStable(0);
                }
                else {
                    self.reaction_network.react();
                    self.stability = Stability::Unstable;
                }
            }
            Stability::SemiStable(count) => {

                if self.reaction_network.get_possible_reactions().is_empty() {
                    self.stability = Stability::Stable;
                }
                else if self.reaction_network.get_possible_reactions().is_subset(self.reaction_network.get_null_adjacent_reactions()) {
                    if count < MAX_SEMI_STABLE_CYCLES {
                        self.reaction_network.react();
                        self.stability = Stability::SemiStable(count + 1);
                    }
                    else {
                        self.reaction_network.react();
                        self.stability = Stability::Stable;
                    }
                }
                else {
                    self.stability = Stability::Unstable;
                }
            }
            Stability::Stable => {
                self.stability = Stability::Stable;
            }
        }
    }
}

enum Stability {
    Initial, 
    Unstable,
    SemiStable(i32),
    Stable,
}