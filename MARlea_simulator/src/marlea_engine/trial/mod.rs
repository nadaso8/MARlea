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
// This function determines the stability of a reaction network and sets the self.stability enum according 
// to the specific conditions met by evaluating the functions get_possible_reactions and get_null_adjacent_reactions. 
// If neither empty nor subset of null reactions, then the network is unstable because there exists a valid reaction,
// otherwise it's initially stable until it enters into one of these states: SemiStable, Stable or Unstable.
fn step(&mut self) {

    match self.stability {
        
        Stability::Initial => {

            // Check if no reaction is possible
            if self.reaction_network.get_possible_reactions().is_empty() {
                self.stability = Stability::Stable;
            }

            // Check if all possible reactions are adjacent to null reactions
            else if self.reaction_network.get_possible_reactions().is_subset(&self.reaction_network.get_null_adjacent_reactions()) {
                self.stability = Stability::SemiStable(0);
            } 
            
            // Otherwise, the network is unstable
            else {
                self.stability = Stability::Unstable;
            }
        } 

        // When in unstable state, make sure isn't stable. then react and set to unstable or semi-stable
        Stability::Unstable => {

            // check again for no possible reactions, and set as stable
            if self.reaction_network.get_possible_reactions().is_empty() {
                self.stability = Stability::Stable;
            }
            // check if all possible reactions are adjacent to null reactions
            else if self.reaction_network.get_possible_reactions().is_subset(self.reaction_network.get_null_adjacent_reactions()) {
                self.reaction_network.react();
                self.stability = Stability::SemiStable(0);
            }
            // the reaction network is still unstable
            else {
                self.reaction_network.react();
                self.stability = Stability::Unstable;
            }
        }

        // When in semi-stable state, make sure isn't stable. then react and set to unstable or semi-stable
        Stability::SemiStable(count) => {

            // check again for no possible reactions, and set as stable
            if self.reaction_network.get_possible_reactions().is_empty() {
                self.stability = Stability::Stable;

            // check if all possible reactions are adjacent to null reactions and has not reached maximum SemiStable cycles    
            } else if self.reaction_network.get_possible_reactions().is_subset(self.reaction_network.get_null_adjacent_reactions()) && count < MAX_SEMI_STABLE_CYCLES {
                    self.reaction_network.react();
                    self.stability = Stability::SemiStable(count + 1);
            
            // Maximum SemiStable cycles reached, change to stable state
            } else if self.reaction_network.get_possible_reactions().is_subset(self.reaction_network.get_null_adjacent_reactions()) && count >= MAX_SEMI_STABLE_CYCLES {
                    self.reaction_network.react();
                    self.stability = Stability::Stable;
            
            // the reaction network is still unstable
            } else {
                self.stability = Stability::Unstable;
            }
        }

        // network is initially stable and remains stable
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