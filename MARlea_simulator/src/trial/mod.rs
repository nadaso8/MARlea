pub mod reaction_network;

use reaction_network::{ReactionNetwork, reaction::Reaction, reaction::term::species::Species};
use std::collections::{HashSet, HashMap};

#[derive(Clone)]
pub enum TrialState<'calculating_trial_results> {
    Initial(ReactionNetwork<'calculating_trial_results>), 
    Unstable(ReactionNetwork<'calculating_trial_results>),
    SemiStable(ReactionNetwork<'calculating_trial_results>, u32),
    Stable(ReactionNetwork<'calculating_trial_results>),
}

impl<'reacting> TrialState<'reacting> {

    pub fn simulate (&mut self) -> HashMap<&Species, Species> {
        loop {
            match self {
                TrialState::Initial(reaction_network) => {
                    // get possible reactions then determine if reaction_network is unstable, semi stable, or stable.  
                } 
                TrialState::Unstable(reaction_network) => {
                    // get possible reactions then determine if reaction_netowrk is unstabel, semi stable, or stable. if the reaction netowrk is unstable or semi stable choose a possible reaction to react
                }
                TrialState::SemiStable(reaction_network, count) => {
                    // get possible reactions then determine if reaction network is unstable, semi stable, or stable. if the reaction network is semi stable increment counter and choose a possible reaction to react
                }
                TrialState::Stable(reaction_network) => {
                    //get solution from reaction network and return it. 
                }
            }

        }
    }

}

