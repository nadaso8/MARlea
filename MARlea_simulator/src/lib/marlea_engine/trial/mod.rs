/// Author: Marceline Sorensen 
/// Email: nadaso8th@gmail.com 
/// Date: 08/03/2023
/// 
/// # Description
/// This code provides a Trial struct and its implementation for simulating a reaction network.
///  The ReactionNetwork module contains all the necessary details about this network, including species, reactions, and terms.
/// The simulation process is performed using the Simulate() function in the implementation of Trial. 
/// It takes a mutable reference to a trial, evaluates its current status, and runs reactions until the network reaches a stable state.
/// 
/// # Usage
/// To use this code, you need to import the ReactionNetwork module and create an instance of ReactionNetwork<'trial>, then create an instance of Trial by passing this instance as an argument.
/// You can then run simulations on this Trial instance using the simulate() function.
/// It returns a HashMap containing all the species keyd by their references in the stable network solution.


// Import necessary modules 
pub mod reaction_network; // Module that holds our ReactionNetwork details
use std::collections::HashMap; // Hashmap class used collectively within the module
use reaction_network::{ReactionNetwork, reaction::term::species::Species}; // Import relevant classes from ReactionNetwork module

const MAX_SEMI_STABLE_CYCLES: i32 = 99; // maximum number of cycles before a semi-stable network is marked as stable

pub struct Trial {
    reaction_network: ReactionNetwork, // Instance of Reaction Network for this trial
    stability: Stability, // Current stability status of the trial/experiment (initially set to "Initial")
}


#[derive(Eq, PartialEq, Clone)]
pub struct TrialResult {
    pub result: HashMap<Species, Species>,
}

// implementation of hash for result
impl std::hash::Hash for TrialResult {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for entry in self.result.iter() {
            entry.hash(state);
        }      
    }
}


impl Trial {

    /// Generates a new instance of `Trial` by taking in an instance of `ReactionNetwork`.
    pub fn from(reaction_network: ReactionNetwork) -> Self {
        Self {
            reaction_network,
            stability: Stability::Initial,
        }
    }

    /// This function simulates a Reaction Network by running reactions on it until the network
    /// reaches a stable state. It takes it's self as a mutable reference to a Trial instance and returns a 
    /// reference to a HashMap containing the Species instances in the stable network solution.
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference to a Trial instance
    ///
    /// # Returns
    ///
    /// A reference to a HashMap that contains all the `species` after simulation keyd by their references.
    pub fn simulate(&mut self) -> TrialResult {
        let mut step_count = 0; 
        loop{
            step_count += 1; 
            self.step();
             // If a stable state has been reached, return the current network solution
            if let Stability::Stable = self.stability {
                println!("stable after {} steps", step_count);
                return TrialResult{result: self.reaction_network.get_solution().clone()};
            }

        }
    }

    /// Handles progressing the simulation one step further by evaluating the current status of the system and performing reactions if necessary.
    // This function determines the stability of a reaction network and sets the self.stability enum according 
    // to the specific conditions met by evaluating the functions get_possible_reactions and get_null_adjacent_reactions. 
    // If neither empty nor subset of null reactions, then the network is unstable because there exists a valid reaction,
    // otherwise it's initially stable until it enters into one of these states: SemiStable, Stable or Unstable.
    fn step(&mut self) {

        match self.stability {
            
            Stability::Initial => {
                self.reaction_network.react();

                if self.reaction_network.get_possible_reactions().is_empty() {
                    self.stability = Stability::Stable;
                }

                else if self.reaction_network.get_possible_reactions().is_subset(&self.reaction_network.get_null_adjacent_reactions()) {
                    self.stability = Stability::SemiStable(0);
                } 
                
                else {
                    self.stability = Stability::Unstable;
                }
            } 

            Stability::Unstable => {
                self.reaction_network.react();

                if self.reaction_network.get_possible_reactions().is_empty() {
                    self.stability = Stability::Stable;
                }

                else if self.reaction_network.get_possible_reactions().is_subset(self.reaction_network.get_null_adjacent_reactions()) {
                    self.stability = Stability::SemiStable(0);
                }

                else {
                    self.stability = Stability::Unstable;
                }
            }

            Stability::SemiStable(count) => {
                self.reaction_network.react();

                if self.reaction_network.get_possible_reactions().is_empty() {
                    self.stability = Stability::Stable;


                } else if self.reaction_network.get_possible_reactions().is_subset(self.reaction_network.get_null_adjacent_reactions()) && count < MAX_SEMI_STABLE_CYCLES {
                        self.reaction_network.react();
                        self.stability = Stability::SemiStable(count + 1);
                

                } else if self.reaction_network.get_possible_reactions().is_subset(self.reaction_network.get_null_adjacent_reactions()) && count >= MAX_SEMI_STABLE_CYCLES {
                        self.reaction_network.react();
                        self.stability = Stability::Stable;
                

                } else {
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