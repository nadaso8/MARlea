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

use reaction_network::{ReactionNetwork, reaction::term::solution::{Solution}};
use results::TrialResult;
use std::sync::mpsc::SyncSender;

pub mod reaction_network; 
pub mod results;

pub struct Trial {
    reaction_network: ReactionNetwork,
    stability: Stability, 
    max_semi_stable_steps: i32,
    id: usize,
}

impl <'trial_runtime> Trial {

    pub fn from(reaction_network: ReactionNetwork, max_semi_stable_steps_setting: Option<i32>, id: usize) -> Self {
        let max_semi_stable_steps: i32; 
        if let Some(number) = max_semi_stable_steps_setting {
            max_semi_stable_steps = number;
        } else {
            max_semi_stable_steps = 99;
        }
        
        Self {
            reaction_network,
            stability: Stability::Initial,
            max_semi_stable_steps,
            id, 
        }
    }

    pub fn simulate_with_timeline (&mut self, trial_tx: SyncSender<TrialResult>)  {
        let mut step_count = 0; 
        loop{
            step_count += 1; 
            self.step();
            trial_tx.send(TrialResult::TimelineEntry(self.reaction_network.get_solution().clone(), self.id))
                .expect("Reciever thread for trial {} dropped\nShutting down...");
            if let Stability::Stable = self.stability {
                trial_tx.send(TrialResult::StableSolution(self.reaction_network.get_solution().clone(), step_count))
                .expect("Reciever thread for trial {} dropped\nShutting down...");
                return;
            }
        }   
    }

    pub fn simulate(&mut self, trial_tx: SyncSender<TrialResult>) {
        let mut step_count = 0; 
        loop{
            step_count += 1; 
            self.step();
            if let Stability::Stable = self.stability {
                trial_tx.send(TrialResult::StableSolution(self.reaction_network.get_solution().clone(), step_count))
                .expect("Reciever thread for trial {} dropped\nShutting down...");
                return;
            }
        }   
    }

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


                } else if self.reaction_network.get_possible_reactions().is_subset(self.reaction_network.get_null_adjacent_reactions()) && count < self.max_semi_stable_steps {
                        self.reaction_network.react();
                        self.stability = Stability::SemiStable(count + 1);
                

                } else if self.reaction_network.get_possible_reactions().is_subset(self.reaction_network.get_null_adjacent_reactions()) && count >= self.max_semi_stable_steps {
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